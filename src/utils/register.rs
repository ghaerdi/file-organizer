use crate::error::AnyError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

/// One individual file movement performed during an organize run: the original
/// absolute source path and the absolute destination it was moved to.
///
/// `--revert` replays these in reverse to put files back where they came from.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoveRecord {
  /// Absolute path the file was moved from (its original location).
  pub from: String,
  /// Absolute path the file was moved to (inside a category directory).
  pub to: String,
}

/// One complete organize run over a directory: the target directory, whether
/// date-nesting was used, and every file that was actually moved.
///
/// Serialised to JSON as a single line in the register file. `--revert` reads
/// the most recent line matching the directory and moves each file back.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunRecord {
  /// The directory the run organized (absolute).
  pub dir: String,
  /// Whether the run used `--by-date` year/month nesting.
  pub by_date: bool,
  /// Wall-clock time the run completed, for human reference.
  pub timestamp: String,
  /// Every file moved, in move order.
  pub moves: Vec<MoveRecord>,
}

/// File name of the register inside the config directory.
/// A single shared register file lives here; runs are distinguished by their
/// recorded `dir` field.
fn register_file() -> String {
  "register.log".to_string()
}

/// Resolve the absolute path of the register log inside the user's state
/// directory: `$XDG_STATE_HOME/file-organizer/register.log`, or
/// `~/.local/state/file-organizer/register.log` on a typical Linux system.
///
/// A state-directory register persists across reboots (so `--revert` can still
/// undo runs older than the last boot) while staying out of the user's config
/// directory. `--revert` resolves the very same path.
pub fn register_path() -> Result<std::path::PathBuf, AnyError> {
  let dirs = directories::ProjectDirs::from("", "", "file-organizer")
    .ok_or(AnyError::NoRegisterDir)?;
  let state = dirs.state_dir().ok_or(AnyError::NoRegisterDir)?;
  Ok(state.join(register_file()))
}

/// Serialise a [`RunRecord`] into a single JSON line (no trailing newline).
fn encode(record: &RunRecord) -> Result<String, AnyError> {
  serde_json::to_string(record)
    .map_err(|e| AnyError::RegisterParse(e.to_string()))
}

/// Parse a single line of the register back into a [`RunRecord`].
fn decode(line: &str) -> Result<RunRecord, AnyError> {
  let trimmed = line.trim();
  if trimmed.is_empty() {
    return Err(AnyError::RegisterParse("empty line".to_string()));
  }
  serde_json::from_str(trimmed)
    .map_err(|e| AnyError::RegisterParse(e.to_string()))
}

/// Append a run record to the register, creating the log when it does not
/// exist yet.
pub fn append_run(record: &RunRecord) -> Result<(), AnyError> {
  let path = register_path()?;
  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent).map_err(AnyError::RegisterWrite)?;
  }
  let mut line = encode(record)?;
  line.push('\n');
  let mut f = fs::OpenOptions::new()
    .create(true)
    .append(true)
    .open(&path)
    .map_err(AnyError::RegisterWrite)?;
  f.write_all(line.as_bytes())
    .map_err(AnyError::RegisterWrite)
}

/// Read every recorded run for `dir`, oldest first.
fn runs_for(dir: &str, path: &Path) -> Result<Vec<RunRecord>, AnyError> {
  if !path.exists() {
    return Ok(Vec::new());
  }
  let contents = fs::read_to_string(path)
    .map_err(|source| AnyError::RegisterRead(source.to_string()))?;
  let mut out = Vec::new();
  for line in contents.lines() {
    let record = decode(line)?;
    if record.dir == dir {
      out.push(record);
    }
  }
  Ok(out)
}

/// The most recent recorded run for `dir`, if any.
fn last_run(dir: &str, path: &Path) -> Result<Option<RunRecord>, AnyError> {
  Ok(runs_for(dir, path)?.pop())
}

/// Undo the most recent organize run on `dir` by reading the register at its
/// default temp location.
pub fn revert_last(dir: &str) -> Result<usize, AnyError> {
  let path = register_path()?;
  revert_last_at(dir, &path)
}

/// Undo the most recent run on `dir` recorded in the register file at `path`.
/// Moves every file back from its recorded destination to its original source,
/// removes directory branches that the revert left empty, then removes the run
/// so a subsequent revert undoes the run before it.
/// Destinations are restored in reverse order so earlier moves do not displace
/// later ones that shared a directory. When a revert target is already
/// occupied, the file is given a numeric suffix rather than overwriting, so a
/// revert never destroys data.
fn revert_last_at(dir: &str, path: &Path) -> Result<usize, AnyError> {
  let record = last_run(dir, path)?.ok_or(AnyError::NothingToRevert)?;

  // Directory that received moved files, and may be left empty by the revert.
  // Removed after restoring if it ends up empty (including by-date nesting).
  let mut candidate_dirs: Vec<std::path::PathBuf> = Vec::new();

  // Restore newest-first: a later file landed after an earlier one inside the
  // same category dir, so undoing the last move first keeps names unambiguous.
  let mut moved_back = 0usize;
  for entry in record.moves.iter().rev() {
    let src = Path::new(&entry.to);
    let dst = Path::new(&entry.from);
    if !src.exists() {
      // Already moved or removed by the user; leave it alone.
      continue;
    }
    let parent = dst.parent().ok_or_else(|| AnyError::FileName {
      path: dst.to_path_buf(),
    })?;
    fs::create_dir_all(parent).map_err(|source| AnyError::RevertCreateDir {
      name: parent.to_string_lossy().into_owned(),
      source,
    })?;
    let resolved = revert_dest(
      parent,
      dst.file_name().and_then(|n| n.to_str()).unwrap_or(""),
    );
    fs::rename(src, &resolved).map_err(|source| AnyError::RevertMoveFile {
      to: resolved.clone(),
      source,
    })?;
    moved_back = moved_back.saturating_add(1);
    // The folder the file was moved from (its category / by-date subfolder) may
    // now be empty; remember it for removal once every move is undone.
    if let Some(from_dir) = Path::new(&entry.to).parent() {
      candidate_dirs.push(from_dir.to_path_buf());
    }
  }

  remove_empty_dirs(dir, &candidate_dirs);

  remove_run(path, &record)?;
  Ok(moved_back)
}

/// Remove directories under `root` that ended up empty after the revert. Only
/// directories touched by the reverted moves (and their now-empty parents up to
/// `root`) are considered, so a pre-existing or still-populated directory is
/// never touched. `fs::remove_dir` only succeeds on empty directories, making
fn remove_empty_dirs(root: &str, start: &[std::path::PathBuf]) {
  let root_path = Path::new(root);
  let mut stack: Vec<std::path::PathBuf> = start.to_vec();
  while let Some(dir) = stack.pop() {
    // Never descend above (or outside) the organized root.
    if !dir.starts_with(root_path) || dir == root_path {
      continue;
    }
    if fs::remove_dir(&dir).is_ok() {
      // Parent may have become empty too; check the next level up.
      if let Some(parent) = dir.parent() {
        stack.push(parent.to_path_buf());
      }
    }
  }
}

/// Remove exactly one run record from the register file. Rebuilds the file
/// from the remaining lines so ordering and other runs are preserved.
fn remove_run(path: &Path, target: &RunRecord) -> Result<(), AnyError> {
  if !path.exists() {
    return Err(AnyError::NothingToRevert);
  }
  let contents = fs::read_to_string(path)
    .map_err(|source| AnyError::RegisterRead(source.to_string()))?;
  let mut remaining = Vec::with_capacity(contents.lines().count());
  let mut removed = false;
  for line in contents.lines() {
    match decode(line) {
      Ok(r) if !removed && r == *target => removed = true,
      Ok(r) => remaining.push(encode(&r)?),
      // A line that cannot parse is preserved verbatim so the register is not
      // silently truncated on a half-written line.
      Err(_) => remaining.push(line.to_string()),
    }
  }
  let mut body = remaining.join("\n");
  if !body.is_empty() {
    body.push('\n');
  }
  fs::write(path, body).map_err(AnyError::RegisterPrune)?;
  Ok(())
}

/// Return `{dir}/{file_name}` unless it already exists, in which case append a
/// `-1`, `-2`, ... suffix (before the extension) until a free name is found.
/// Mirrors the forward-path collision handling so a revert never overwrites.
fn revert_dest(dir: &Path, file_name: &str) -> String {
  let plain = dir.join(file_name);
  if !plain.exists() {
    return plain.to_string_lossy().into_owned();
  }
  let path = Path::new(file_name);
  let stem = path
    .file_stem()
    .and_then(|s| s.to_str())
    .unwrap_or(file_name);
  let ext = path.extension().and_then(|e| e.to_str());
  for n in 1.. {
    let name =
      ext.map_or_else(|| format!("{stem}-{n}"), |e| format!("{stem}-{n}.{e}"));
    let candidate = dir.join(&name);
    if !candidate.exists() {
      return candidate.to_string_lossy().into_owned();
    }
  }
  // `range 1..` never ends, so the loop always returns; this is unreachable.
  String::new()
}

#[cfg(test)]
mod test {
  #![allow(clippy::expect_used, clippy::unwrap_used, clippy::indexing_slicing)]
  use super::*;
  use crate::test_utils::TempDir;

  fn temp_register(
    dir: &RunRecord,
  ) -> (TempDir, std::path::PathBuf, RunRecord) {
    let temp = TempDir::new("register");
    let path = temp.path().join("register.log");
    let record = dir.clone();
    (temp, path, record)
  }

  #[test]
  fn round_trip_serializes_and_parses() {
    let record = RunRecord {
      dir: "/tmp/x".to_string(),
      by_date: true,
      timestamp: "2024-01-01T12:00:00Z".to_string(),
      moves: vec![MoveRecord {
        from: "/tmp/x/a.txt".to_string(),
        to: "/tmp/x/Text/a.txt".to_string(),
      }],
    };
    let encoded = encode(&record).expect("encode");
    let decoded = decode(&encoded).expect("decode");
    assert_eq!(decoded, record, "round-trip must preserve the record");
  }

  #[test]
  fn last_run_returns_most_recent_for_dir() {
    let (temp, path, _) = temp_register(&RunRecord {
      dir: "/d".to_string(),
      by_date: false,
      timestamp: "t".to_string(),
      moves: Vec::new(),
    });
    let a = RunRecord {
      dir: "/d".to_string(),
      by_date: false,
      timestamp: "older".to_string(),
      moves: Vec::new(),
    };
    let b = RunRecord {
      dir: "/d".to_string(),
      by_date: false,
      timestamp: "newer".to_string(),
      moves: Vec::new(),
    };
    fs::write(
      &path,
      format!("{}\n{}\n", encode(&a).unwrap(), encode(&b).unwrap()),
    )
    .unwrap();
    let got = last_run("/d", &path).expect("read").expect("a run exists");
    assert_eq!(got.timestamp, "newer");
    let other = last_run("/other", &path).expect("read");
    assert!(other.is_none(), "no run for a different dir");
    drop(temp);
  }

  #[test]
  fn remove_run_prunes_only_matching_record() {
    let (temp, path, _) = temp_register(&RunRecord {
      dir: "/d".to_string(),
      by_date: false,
      timestamp: "t".to_string(),
      moves: Vec::new(),
    });
    let a = RunRecord {
      dir: "/d".to_string(),
      by_date: false,
      timestamp: "a".to_string(),
      moves: Vec::new(),
    };
    let b = RunRecord {
      dir: "/d".to_string(),
      by_date: false,
      timestamp: "b".to_string(),
      moves: Vec::new(),
    };
    fs::write(
      &path,
      format!("{}\n{}\n", encode(&a).unwrap(), encode(&b).unwrap()),
    )
    .unwrap();
    let runs_before = runs_for("/d", &path).expect("read");
    assert_eq!(runs_before.len(), 2);
    remove_run(&path, &a).expect("prune");
    let runs_after = runs_for("/d", &path).expect("read");
    assert_eq!(runs_after.len(), 1);
    assert_eq!(runs_after[0].timestamp, "b");
    drop(temp);
  }

  #[test]
  fn revert_last_moves_files_back_and_prunes_run() {
    let temp = TempDir::new("revert");
    let dir = temp.path().join("dl");
    fs::create_dir(&dir).unwrap();
    let cat_text = dir.join("Text");
    let cat_img = dir.join("Image");
    fs::create_dir(&cat_text).unwrap();
    fs::create_dir(&cat_img).unwrap();
    let src_txt = dir.join("a.txt");
    let src_img = dir.join("b.png");
    fs::write(&src_txt, "txt").unwrap();
    fs::write(&src_img, "png").unwrap();
    let dst_txt = cat_text.join("a.txt");
    let dst_img = cat_img.join("b.png");
    fs::rename(&src_txt, &dst_txt).unwrap();
    fs::rename(&src_img, &dst_img).unwrap();

    let record = RunRecord {
      dir: dir.to_str().unwrap().to_string(),
      by_date: false,
      timestamp: "now".to_string(),
      moves: vec![
        MoveRecord {
          from: src_txt.to_string_lossy().into_owned(),
          to: dst_txt.to_string_lossy().into_owned(),
        },
        MoveRecord {
          from: src_img.to_string_lossy().into_owned(),
          to: dst_img.to_string_lossy().into_owned(),
        },
      ],
    };
    let log = temp.path().join("register.log");
    fs::write(&log, format!("{}\n", encode(&record).unwrap())).unwrap();

    let moved =
      revert_last_at(dir.to_str().unwrap(), &log).expect("revert succeeds");
    assert_eq!(moved, 2, "both files moved back");
    assert!(dir.join("a.txt").exists(), "a.txt back at the root");
    assert!(dir.join("b.png").exists(), "b.png back at the root");
    assert!(!dst_txt.exists(), "Text/a.txt emptied");
    assert!(!dst_img.exists(), "Image/b.png emptied");
    // The run is pruned, so a second revert finds nothing to undo.
    assert!(revert_last_at(dir.to_str().unwrap(), &log).is_err());
  }

  #[test]
  fn revert_does_not_overwrite_an_occupied_target() {
    let temp = TempDir::new("revert_collide");
    let dir = temp.path().join("dl");
    fs::create_dir(&dir).unwrap();
    let cat = dir.join("Text");
    fs::create_dir(&cat).unwrap();
    let src = dir.join("a.txt");
    fs::write(&src, "moved").unwrap();
    let dst = cat.join("a.txt");
    fs::rename(&src, &dst).unwrap();

    let record = RunRecord {
      dir: dir.to_str().unwrap().to_string(),
      by_date: false,
      timestamp: "now".to_string(),
      moves: vec![MoveRecord {
        from: src.to_string_lossy().into_owned(),
        to: dst.to_string_lossy().into_owned(),
      }],
    };
    let log = temp.path().join("register.log");
    fs::write(&log, format!("{}\n", encode(&record).unwrap())).unwrap();
    // A new file now occupies the original location.
    fs::write(&src, "new occupant").unwrap();

    let moved =
      revert_last_at(dir.to_str().unwrap(), &log).expect("revert succeeds");
    assert_eq!(moved, 1);
    // Original occupant untouched.
    assert_eq!(
      fs::read_to_string(&src).unwrap(),
      "new occupant",
      "existing file must not be overwritten"
    );
    // The reverted file landed under a suffixed name instead.
    assert!(dir.join("a-1.txt").exists(), "reverted file given a suffix");
    assert!(!dst.exists(), "category copy moved out");
  }

  #[test]
  fn revert_removes_empty_dirs_and_nested_by_date_branches() {
    let temp = TempDir::new("revert_cleanup");
    let dir = temp.path().join("dl");
    fs::create_dir(&dir).unwrap();
    // Two categories: Text/a.txt (flat) and Image/2024/08/b.png (by-date nest).
    let text = dir.join("Text");
    let img_y = dir.join("Image/2024");
    fs::create_dir(&text).unwrap();
    let img_m = img_y.join("08");
    fs::create_dir_all(&img_m).unwrap();
    let src_a = dir.join("a.txt");
    let src_b = dir.join("b.png");
    fs::write(&src_a, "a").unwrap();
    fs::write(&src_b, "b").unwrap();
    let dst_a = text.join("a.txt");
    let dst_b = img_m.join("b.png");
    fs::rename(&src_a, &dst_a).unwrap();
    fs::rename(&src_b, &dst_b).unwrap();

    let record = RunRecord {
      dir: dir.to_str().unwrap().to_string(),
      by_date: true,
      timestamp: "now".to_string(),
      moves: vec![
        MoveRecord {
          from: src_a.to_string_lossy().into_owned(),
          to: dst_a.to_string_lossy().into_owned(),
        },
        MoveRecord {
          from: src_b.to_string_lossy().into_owned(),
          to: dst_b.to_string_lossy().into_owned(),
        },
      ],
    };
    let log = temp.path().join("register.log");
    fs::write(&log, format!("{}\n", encode(&record).unwrap())).unwrap();

    let moved =
      revert_last_at(dir.to_str().unwrap(), &log).expect("revert succeeds");
    assert_eq!(moved, 2);
    assert!(dir.join("a.txt").exists());
    assert!(dir.join("b.png").exists());
    // Both the month and the now-empty parent year dir are gone.
    assert!(!text.exists(), "flat Text removed when emptied");
    assert!(!img_m.exists(), "Image/2024/08 removed");
    assert!(!img_y.exists(), "Image/2024 removed as its only child went");
    assert!(!dir.join("Image").exists(), "Image removed when emptied");
  }

  #[test]
  fn revert_keeps_dirs_that_still_hold_other_files() {
    let temp = TempDir::new("revert_keeps");
    let dir = temp.path().join("dl");
    fs::create_dir(&dir).unwrap();
    let text = dir.join("Text");
    fs::create_dir(&text).unwrap();
    let src = dir.join("a.txt");
    fs::write(&src, "a").unwrap();
    let dst = text.join("a.txt");
    fs::rename(&src, &dst).unwrap();
    // An unrelated file stays in the category dir.
    let keeper = text.join("keep.txt");
    fs::write(&keeper, "keep").unwrap();

    let record = RunRecord {
      dir: dir.to_str().unwrap().to_string(),
      by_date: false,
      timestamp: "now".to_string(),
      moves: vec![MoveRecord {
        from: src.to_string_lossy().into_owned(),
        to: dst.to_string_lossy().into_owned(),
      }],
    };
    let log = temp.path().join("register.log");
    fs::write(&log, format!("{}\n", encode(&record).unwrap())).unwrap();

    let moved =
      revert_last_at(dir.to_str().unwrap(), &log).expect("revert succeeds");
    assert_eq!(moved, 1);
    assert!(dir.join("a.txt").exists(), "file moved back to the root");
    assert!(text.is_dir(), "non-empty Text dir is left alone");
    assert!(keeper.exists(), "unrelated file untouched");
  }
}
