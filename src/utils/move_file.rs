use crate::error::AnyError;
use std::fs;
use std::path::Path;

pub fn move_file(from: &Path, to: &str) -> Result<(), AnyError> {
  let file_name =
    from.file_name().and_then(|n| n.to_str()).ok_or_else(|| {
      AnyError::FileName {
        path: from.to_path_buf(),
      }
    })?;
  let to = format!("{to}/{file_name}");

  fs::rename(from, &to).map_err(|source| AnyError::MoveFile {
    to: to.clone(),
    source,
  })?;
  println!("moved to");
  println!("{to}");
  println!();
  Ok(())
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

    move_file(&src, dst_dir.to_str().expect("utf-8"))
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
    let err = move_file(&src, missing.to_str().expect("utf-8"))
      .expect_err("moving into nonexistent dir must fail");
    assert!(matches!(err, AnyError::MoveFile { .. }));
    // Source untouched.
    assert!(src.exists());
  }
}
