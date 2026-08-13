use crate::error::AnyError;
use crate::{globals, utils};
use std::env::consts;
use std::fs;

/// Aggregate result of an [`organize`] pass.
pub struct OrganizeSummary {
  /// Number of files actually moved (or previewed for move when dry-running).
  pub moved: usize,
  /// Number of entries left untouched (symlinks, category dirs, no-extension
  /// files on Windows).
  pub skipped: usize,
  /// Relative destination paths (e.g. `Text/a.txt` or `Text/2024/08/a.txt`)
  /// for every file that was or would be moved, in actual move order.
  pub destinations: Vec<String>,
  /// Absolute source→destination pairs for every file actually moved. Empty
  /// during a dry run (nothing moved on disk). Used to build the register that
  /// `--revert` replays.
  pub moves: Vec<utils::MoveRecord>,
}

pub fn organize(
  path: &str,
  dry_run: bool,
  by_date: bool,
) -> Result<OrganizeSummary, AnyError> {
  let main_dir = fs::read_dir(path).map_err(AnyError::OpenDir)?;
  let map = utils::files_extension();
  let dirs = globals::DIRS.to_vec();
  let other = globals::DIRS.last().ok_or(AnyError::NoOtherCategory)?;

  let mut summary = OrganizeSummary {
    moved: 0,
    skipped: 0,
    destinations: Vec::new(),
    moves: Vec::new(),
  };

  for entry in main_dir {
    let entry = entry.map_err(AnyError::ReadEntry)?.path();
    let name = entry.file_name().and_then(|n| n.to_str()).ok_or_else(|| {
      AnyError::FileName {
        path: entry.clone(),
      }
    })?;

    if dirs.contains(&name) {
      summary.skipped = summary.skipped.saturating_add(1);
      continue;
    }

    // Never follow symlinks: a link could point outside the tree (or dangle),
    // and moving it would silently relocate an external file, so leave it alone.
    let is_symlink =
      fs::symlink_metadata(&entry).is_ok_and(|m| m.file_type().is_symlink());
    if is_symlink {
      summary.skipped = summary.skipped.saturating_add(1);
      continue;
    }

    let destination = match entry.extension().and_then(|e| e.to_str()) {
      Some(ext) => map
        .get(ext)
        .map_or_else(|| format!("{path}/{other}"), |d| format!("{path}/{d}")),
      None if consts::OS != "windows" => format!("{path}/{other}"),
      None => {
        summary.skipped = summary.skipped.saturating_add(1);
        continue;
      },
    };
    let dest = utils::move_file(&entry, &destination, dry_run, by_date)?;
    summary.moved = summary.moved.saturating_add(1);
    summary
      .destinations
      .push(relative_to(dest.as_str(), path).to_string());
    if !dry_run {
      summary.moves.push(utils::MoveRecord {
        from: entry.to_string_lossy().into_owned(),
        to: dest.clone(),
      });
    }
  }

  // `destinations` is populated in filesystem `read_dir` order, which is
  // non-deterministic across platforms/filesystems. Sort it so the reported
  // output and the rendered tree are stable regardless of iteration order.
  summary.destinations.sort();

  Ok(summary)
}

/// Strip the organize root prefix so a destination such as `/tmp/x/Text/a.txt`
/// is rendered relative to that root as `Text/a.txt`. Falls back to the absolute
/// path when the destination is not under the root.
fn relative_to<'a>(dest: &'a str, root: &str) -> &'a str {
  let prefix = format!("{root}/");
  dest.strip_prefix(&prefix).unwrap_or(dest)
}

#[cfg(test)]
mod test {
  #![allow(clippy::expect_used, clippy::unwrap_used, clippy::indexing_slicing)]
  use super::*;
  use crate::test_utils::TempDir;
  use std::fs::File;
  use std::path::Path;

  #[test]
  fn organize_files() {
    let temp = TempDir::new("organize");
    let dir = temp.path().to_str().expect("temp path is utf-8");
    let files = vec![
      "foo.txt",
      "foo.png",
      "foo.mp3",
      "foo.mp4",
      "foo.zip",
      "foo.exe",
      "foo.bar",
      "foo_noext",
    ];
    create_files(dir, &files);

    organize(dir, false, false).expect("unable to organize");

    let mut expect = vec![
      "Text/foo.txt",
      "Image/foo.png",
      "Audio/foo.mp3",
      "Video/foo.mp4",
      "Compressed/foo.zip",
      "Executable/foo.exe",
      "Other/foo.bar",
    ];
    #[cfg(unix)]
    expect.push("Other/foo_noext");
    // On Windows, extension-less files are left untouched (see organize()),
    // so `foo_noext` stays at the root; assert that instead.
    #[cfg(windows)]
    {
      assert!(Path::new(&format!("{dir}/foo_noext")).exists());
    }
    assert_all(dir, expect);
    // Categories that received no files must not be materialised.
    assert!(
      !Path::new(&format!("{dir}/Data")).exists(),
      "Data stays empty"
    );
    assert!(
      !Path::new(&format!("{dir}/Font")).exists(),
      "Font stays empty"
    );
    assert!(
      !Path::new(&format!("{dir}/Disk")).exists(),
      "Disk stays empty"
    );
  }

  #[test]
  fn empty_source_creates_no_category_dirs() {
    let temp = TempDir::new("organize_empty");
    let dir = temp.path().to_str().expect("temp path is utf-8");

    let summary = organize(dir, false, false).expect("organize empty dir");
    assert_eq!(summary.moved, 0, "no files to move");
    assert_eq!(summary.skipped, 0, "no dirs/symlinks to skip");
    // No category directory should exist at all after a no-op run.
    for dir_name in globals::DIRS {
      assert!(
        !Path::new(&format!("{dir}/{dir_name}")).exists(),
        "empty category {dir_name} must not be created",
        dir_name = dir_name
      );
    }
  }

  #[test]
  fn organize_idempotent() {
    let temp = TempDir::new("organize_idem");
    let dir = temp.path().to_str().expect("temp path is utf-8");
    // No pre-created category dirs — organize creates them lazily.
    create_files(dir, &["a.txt", "b.png"]);
    organize(dir, false, false).expect("organize #1");
    organize(dir, false, false).expect("organize #2");
    assert!(Path::new(&format!("{dir}/Text/a.txt")).exists());
    assert!(Path::new(&format!("{dir}/Image/b.png")).exists());
  }

  #[test]
  fn organize_dry_run_moves_nothing() {
    let temp = TempDir::new("organize_dry");
    let dir = temp.path().to_str().expect("temp path is utf-8");
    create_files(dir, &["a.txt", "b.png"]);

    organize(dir, true, false).expect("dry run should succeed");

    // No file should have been relocated into a category directory.
    assert!(
      !Path::new(&format!("{dir}/Text/a.txt")).exists(),
      "dry run must not move files"
    );
    assert!(
      !Path::new(&format!("{dir}/Image/b.png")).exists(),
      "dry run must not move files"
    );
    // Originals remain in place.
    assert!(Path::new(&format!("{dir}/a.txt")).exists());
    assert!(Path::new(&format!("{dir}/b.png")).exists());
  }

  #[test]
  fn organize_by_date_groups_into_year_month_subfolder() {
    let temp = TempDir::new("organize_date");
    let dir = temp.path().to_str().expect("temp path is utf-8");
    create_files(dir, &["a.txt"]);

    // By-date organization removes `a.txt` from the flat root...
    organize(dir, false, true).expect("by-date organize succeeds");
    let root_a_path = format!("{dir}/a.txt");
    let root_a = Path::new(&root_a_path);
    assert!(
      !root_a.exists(),
      "a.txt should be moved out of the flat root by by-date organize"
    );

    // ...and places it under a nested year/month folder inside Text.
    assert!(Path::new(&format!("{dir}/Text")).is_dir());
    let year = fs::read_dir(format!("{dir}/Text")).expect("read Text/ dir");
    let mut found = false;
    for year_entry in year {
      let year_path = year_entry.expect("year entry").path();
      if !year_path.is_dir() {
        continue;
      }
      if year_path.join("a.txt").exists() {
        // year/ only (no month level)
        found = year_path.join("a.txt").exists();
        break;
      }
      if let Ok(months) = fs::read_dir(&year_path) {
        for month_entry in months {
          let month_path = month_entry.expect("month entry").path();
          if month_path.is_dir() && month_path.join("a.txt").exists() {
            found = true;
            break;
          }
        }
      }
    }
    assert!(
      found,
      "a.txt should be found under a Text/<year>/<month>/ nesting"
    );
  }

  #[test]
  fn organize_by_date_with_dry_run_moves_nothing() {
    let temp = TempDir::new("organize_date_dry");
    let dir = temp.path().to_str().expect("temp path is utf-8");
    create_files(dir, &["a.txt", "b.png"]);

    organize(dir, true, true).expect("dry run with by-date succeeds");
  }

  #[test]
  fn organize_summary_counts_moved_files() {
    let temp = TempDir::new("organize_summary");
    let dir = temp.path().to_str().expect("temp path is utf-8");
    create_files(dir, &["a.txt", "b.png", "c.zip"]);

    let summary = organize(dir, false, false).expect("organize succeeds");
    assert_eq!(summary.moved, 3, "three files moved");
    assert_eq!(summary.skipped, 0, "no pre-existing dirs to skip");
    assert_eq!(
      summary.destinations,
      vec![
        "Compressed/c.zip".to_string(),
        "Image/b.png".to_string(),
        "Text/a.txt".to_string(),
      ],
      "destinations sorted alphabetically (deterministic output)"
    );
  }

  #[cfg(unix)]
  #[test]
  fn organize_skips_symlinks_and_counts_them() {
    use std::os::unix::fs::symlink;

    let temp = TempDir::new("organize_symlink");
    let dir_path = temp.path().join("org");
    fs::create_dir(&dir_path).expect("create org dir");
    let dir = dir_path.to_str().expect("temp path is utf-8");

    // Target sits OUTSIDE the organized folder so the link is the only `.txt`
    // entry inside it.
    let target = temp.path().join("outside.txt");
    fs::write(&target, "keep me").expect("write target");
    let link = dir_path.join("link.txt");
    symlink(&target, &link).expect("create symlink");
    let summary = organize(dir, false, false).expect("organize succeeds");
    assert_eq!(summary.moved, 0, "no real file moved");
    // The symlink must not be relocated.
    assert!(
      link.exists()
        && fs::symlink_metadata(&link)
          .expect("stat link")
          .file_type()
          .is_symlink(),
      "symlink stays in place and remains a symlink"
    );
    // Only the symlink is skipped (no pre-created category dirs).
    assert_eq!(summary.skipped, 1, "one symlink skipped");
  }

  fn create_files(dir: &str, files: &[&str]) {
    for f in files {
      File::create(format!("{dir}/{f}")).expect("unable to create file");
    }
  }

  fn assert_all(dir: &str, files: Vec<&str>) {
    for f in files {
      assert!(Path::new(&format!("{dir}/{f}")).exists());
    }
  }
}
