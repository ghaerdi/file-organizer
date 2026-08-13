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
  use std::fs;
  use std::path::Path;

  // create a dir and then call the create_dirs function, check if exist each directory and then
  // remove all the dirs.
  #[test]
  fn create_directories() {
    let main_dir = "./create_dirs";
    fs::create_dir(&main_dir).expect("unable to create");
    create_dirs(&main_dir).expect("unable to create subdirectories");

    assert!(Path::new("./create_dirs/Text").exists());
    assert!(Path::new("./create_dirs/Image").exists());
    assert!(Path::new("./create_dirs/Audio").exists());
    assert!(Path::new("./create_dirs/Video").exists());
    assert!(Path::new("./create_dirs/Compressed").exists());
    assert!(Path::new("./create_dirs/Executable").exists());

    fs::remove_dir_all("./create_dirs").expect("unable to remove");
  }
}
