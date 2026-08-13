use crate::error::AnyError;
use crate::globals;
use std::fs;
use std::path::Path;

pub fn create_dirs(dir: &str) -> Result<(), AnyError> {
  for value in globals::DIRS {
    let subdir = format!("{dir}/{value}");
    let path = Path::new(&subdir);

    if !path.exists() {
      fs::create_dir(path).map_err(|source| AnyError::CreateDir {
        name: String::from(value),
        source,
      })?;
      println!("{value} directory created");
    }
  }
  Ok(())
}

#[cfg(test)]
mod test {
  #![allow(clippy::expect_used, clippy::unwrap_used, clippy::indexing_slicing)]
  use super::*;
  use crate::test_utils::TempDir;

  // create a temp dir, then call create_dirs, assert each subdirectory exists, and let
  // the TempDir guard remove everything on drop.
  #[test]
  fn create_directories() {
    let temp = TempDir::new("create_dirs");
    let main_dir = temp.path().to_str().expect("temp path is utf-8");
    create_dirs(main_dir).expect("unable to create subdirectories");

    assert!(temp.path().join("Text").exists());
    assert!(temp.path().join("Image").exists());
    assert!(temp.path().join("Audio").exists());
    assert!(temp.path().join("Video").exists());
    assert!(temp.path().join("Compressed").exists());
    assert!(temp.path().join("Executable").exists());
  }
}
