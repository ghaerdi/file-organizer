use crate::error::AnyError;
use std::fs;
use std::path::Path;

/// Move `from` into directory `to`, keeping the original file name unless it
/// collides with an existing file (in which case a numeric suffix is added so
/// nothing is ever overwritten).
///
/// When `dry_run` is true, the destination is computed and printed but nothing
/// is renamed, so the caller can preview what would happen.
pub fn move_file(from: &Path, to: &str, dry_run: bool) -> Result<(), AnyError> {
  let file_name =
    from.file_name().and_then(|n| n.to_str()).ok_or_else(|| {
      AnyError::FileName {
        path: from.to_path_buf(),
      }
    })?;
  let dest = unique_dest(to, file_name);

  if dry_run {
    println!("would move to");
    println!("{dest}");
    println!();
    return Ok(());
  }

  fs::rename(from, &dest).map_err(|source| AnyError::MoveFile {
    to: dest.clone(),
    source,
  })?;
  println!("moved to");
  println!("{dest}");
  println!();
  Ok(())
}

/// Build a destination path `{dir}/{file_name}` that does not collide with an
/// existing file. If `{dir}/{file_name}` already exists, the name is suffixed
/// with `-1`, `-2`, ... before the extension (or at the end when there is no
/// extension) until a free name is found, so incoming files never overwrite
/// existing ones.
fn unique_dest(dir: &str, file_name: &str) -> String {
  let plain = format!("{dir}/{file_name}");
  if !Path::new(&plain).exists() {
    return plain;
  }

  let path = Path::new(file_name);
  let stem = path
    .file_stem()
    .and_then(|s| s.to_str())
    .unwrap_or(file_name);
  let ext = path.extension().and_then(|e| e.to_str());

  for n in 1.. {
    let candidate = ext.map_or_else(
      || format!("{dir}/{stem}-{n}"),
      |ext| format!("{dir}/{stem}-{n}.{ext}"),
    );
    if !Path::new(&candidate).exists() {
      return candidate;
    }
  }
  String::new()
}

#[cfg(test)]
mod test {
  #![allow(clippy::expect_used, clippy::unwrap_used, clippy::indexing_slicing)]
  use super::*;
  use crate::test_utils::TempDir;
  use std::fs::File;

  #[test]
  fn moves_file_into_target_dir() {
    let temp = TempDir::new("move");
    let src = temp.path().join("a.txt");
    File::create(&src).expect("create source file");

    let dst_dir = temp.path().join("dst");
    fs::create_dir(&dst_dir).expect("create target dir");

    move_file(&src, dst_dir.to_str().expect("utf-8"), false)
      .expect("move should succeed");

    let moved = dst_dir.join("a.txt");
    assert!(moved.exists(), "file should exist in target dir");
    assert!(!src.exists(), "source file should be gone");
  }

  #[test]
  fn moving_to_missing_dir_errors() {
    let temp = TempDir::new("move_err");
    let src = temp.path().join("a.txt");
    File::create(&src).expect("create source file");

    let missing = temp.path().join("does_not_exist");
    let err = move_file(&src, missing.to_str().expect("utf-8"), false)
      .expect_err("moving into nonexistent dir must fail");
    assert!(matches!(err, AnyError::MoveFile { .. }));
    // Source untouched.
    assert!(src.exists());
  }

  #[test]
  fn dry_run_leaves_source_and_dest_untouched() {
    let temp = TempDir::new("move_dry");
    let src = temp.path().join("a.txt");
    fs::write(&src, "content").expect("create source file");

    let dst_dir = temp.path().join("dst");
    fs::create_dir(&dst_dir).expect("create target dir");

    move_file(&src, dst_dir.to_str().expect("utf-8"), true)
      .expect("dry run should succeed");

    // Nothing moved.
    assert!(src.exists(), "source must remain during dry run");
    assert!(
      !dst_dir.join("a.txt").exists(),
      "no file should land in target during dry run"
    );
    assert_eq!(
      fs::read_to_string(&src).expect("read source"),
      "content",
      "source content unchanged"
    );
  }

  #[test]
  fn collision_renames_incoming_file_instead_of_overwriting() {
    let temp = TempDir::new("move_collide");
    let dst_dir = temp.path().join("dst");
    fs::create_dir(&dst_dir).expect("create target dir");

    // Pre-existing file that must not be overwritten.
    fs::write(dst_dir.join("a.txt"), "original")
      .expect("create existing target");

    let src = temp.path().join("a.txt");
    fs::write(&src, "new").expect("create source file");

    move_file(&src, dst_dir.to_str().expect("utf-8"), false)
      .expect("move should succeed");

    // Original preserved.
    assert_eq!(
      fs::read_to_string(dst_dir.join("a.txt")).expect("read original"),
      "original",
      "existing file must not be overwritten"
    );
    // Source moved somewhere into dst, not left behind.
    assert!(!src.exists(), "source file should be gone");
    // Exactly one extra file landed in dst with a distinct name.
    let landed: Vec<_> = fs::read_dir(&dst_dir)
      .expect("read dst")
      .map(|e| e.expect("entry").file_name().into_string().expect("name"))
      .collect();
    assert_eq!(landed.len(), 2, "original + renamed incoming: {landed:?}");
    let incoming = landed
      .iter()
      .find(|n| n.as_str() != "a.txt")
      .expect("a renamed copy exists");
    assert_eq!(
      fs::read_to_string(dst_dir.join(incoming)).expect("read incoming"),
      "new",
      "incoming content preserved under renamed path"
    );
  }

  #[test]
  fn collision_then_repeated_move_keeps_original_intact() {
    let temp = TempDir::new("move_collide2");
    let dst_dir = temp.path().join("dst");
    fs::create_dir(&dst_dir).expect("create target dir");
    fs::write(dst_dir.join("a.txt"), "original")
      .expect("create existing target");

    let src = temp.path().join("a.txt");
    fs::write(&src, "copy1").expect("create source");
    move_file(&src, dst_dir.to_str().expect("utf-8"), false).expect("move 1");

    let src2 = temp.path().join("a.txt");
    fs::write(&src2, "copy2").expect("create source 2");
    move_file(&src2, dst_dir.to_str().expect("utf-8"), false).expect("move 2");

    // Original never touched across both collisions.
    assert_eq!(
      fs::read_to_string(dst_dir.join("a.txt")).expect("read original"),
      "original"
    );
    // Two distinct renamed copies, no overwrite between them.
    let names: Vec<String> = fs::read_dir(&dst_dir)
      .expect("read dst")
      .map(|e| e.expect("entry").file_name().into_string().expect("name"))
      .collect();
    assert_eq!(names.len(), 3, "original + 2 copies: {names:?}");
  }
}
