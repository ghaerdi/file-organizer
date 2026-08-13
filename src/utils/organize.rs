use crate::{globals, utils};
use std::env::consts;
use std::error::Error;
use std::fs;

pub fn organize(path: &str) -> Result<(), Box<dyn Error>> {
  let main_dir = fs::read_dir(path).map_err(|_| "unable to open directory")?;
  let map = utils::files_extension();
  let dirs = globals::DIRS.to_vec();
  let other = globals::DIRS.last().ok_or("no Other category")?;

  for entry in main_dir {
    let entry = entry.map_err(|_| "unable to read directory entry")?.path();
    let name = entry
      .file_name()
      .and_then(|n| n.to_str())
      .ok_or("unable to determine file name")?;

    if dirs.contains(&name) {
      continue;
    }

    match entry.extension().and_then(|e| e.to_str()) {
      Some(ext) => {
        let dir = map
          .get(ext)
          .map_or_else(|| format!("{path}/{other}"), |d| format!("{path}/{d}"));
        utils::move_file(&entry, &dir)?;
      },
      None if consts::OS != "windows" => {
        let dir = format!("{path}/{other}");
        utils::move_file(&entry, &dir)?;
      },
      None => {},
    }
  }
  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::utils;
  use std::fs;
  use std::fs::File;
  use std::path::Path;

  const DIR: &str = "./.move_file";

  #[test]
  fn organize_files() {
    fs::create_dir(DIR).expect("unable to create dir");
    utils::create_dirs(DIR).expect("unable to create subdirectories");

    let files = vec![
      "foo.txt", "foo.png", "foo.mp3", "foo.mp4", "foo.zip", "foo.exe",
      "foo.bar",
    ];

    create_files(&files);
    organize(DIR).expect("unable to organize");

    let expect = vec![
      "Text/foo.txt",
      "Image/foo.png",
      "Audio/foo.mp3",
      "Video/foo.mp4",
      "Compressed/foo.zip",
      "Executable/foo.exe",
      "Other/foo.bar",
    ];
    assert_all(expect);

    fs::remove_dir_all(DIR).expect("unable to remove");
  }

  fn create_files(files: &[&str]) {
    for f in files {
      let path = format!("{DIR}/{f}");
      File::create(path).expect("unable to create file");
    }
  }

  fn assert_all(files: Vec<&str>) {
    for f in files {
      let path = format!("{DIR}/{f}");
      assert!(Path::new(&path).exists());
    }
  }
}
