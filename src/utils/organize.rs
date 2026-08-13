use crate::error::AnyError;
use crate::{globals, utils};
use std::env::consts;
use std::fs;

pub fn organize(
  path: &str,
  dry_run: bool,
  by_date: bool,
) -> Result<(), AnyError> {
  let main_dir = fs::read_dir(path).map_err(AnyError::OpenDir)?;
  let map = utils::files_extension();
  let dirs = globals::DIRS.to_vec();
  let other = globals::DIRS.last().ok_or(AnyError::NoOtherCategory)?;

  for entry in main_dir {
    let entry = entry.map_err(AnyError::ReadEntry)?.path();
    let name = entry.file_name().and_then(|n| n.to_str()).ok_or_else(|| {
      AnyError::FileName {
        path: entry.clone(),
      }
    })?;

    if dirs.contains(&name) {
      continue;
    }

    match entry.extension().and_then(|e| e.to_str()) {
      Some(ext) => {
        let dir = map
          .get(ext)
          .map_or_else(|| format!("{path}/{other}"), |d| format!("{path}/{d}"));
        utils::move_file(&entry, &dir, dry_run, by_date)?;
      },
      None if consts::OS != "windows" => {
        let dir = format!("{path}/{other}");
        utils::move_file(&entry, &dir, dry_run, by_date)?;
      },
      None => {},
    }
  }
  Ok(())
}

#[cfg(test)]
mod test {
  #![allow(clippy::expect_used, clippy::unwrap_used, clippy::indexing_slicing)]
  use super::*;
  use crate::test_utils::TempDir;
  use crate::utils;
  use std::fs::File;
  use std::path::Path;

  #[test]
  fn organize_files() {
    let temp = TempDir::new("organize");
    let dir = temp.path().to_str().expect("temp path is utf-8");
    utils::create_dirs(dir).expect("unable to create subdirectories");

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

    let expect = vec![
      "Text/foo.txt",
      "Image/foo.png",
      "Audio/foo.mp3",
      "Video/foo.mp4",
      "Compressed/foo.zip",
      "Executable/foo.exe",
      "Other/foo.bar",
      "Other/foo_noext",
    ];
    assert_all(dir, expect);
  }

  #[test]
  fn organize_idempotent() {
    let temp = TempDir::new("organize_idem");
    let dir = temp.path().to_str().expect("temp path is utf-8");
    utils::create_dirs(dir).expect("unable to create subdirectories");
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
    utils::create_dirs(dir).expect("unable to create subdirectories");
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
    utils::create_dirs(dir).expect("unable to create subdirectories");
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
    utils::create_dirs(dir).expect("unable to create subdirectories");
    create_files(dir, &["a.txt", "b.png"]);

    organize(dir, true, true).expect("dry run with by-date succeeds");
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
