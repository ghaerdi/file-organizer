use crate::{globals, utils};
use std::fs;

pub fn organize(path: String) {
  let main_dir = fs::read_dir(&path).expect("unable to open");
  let map = utils::files_extension();
  let dirs = globals::DIRS.to_vec();

  for entry in main_dir {
    let entry = entry.unwrap().path();
    let name = entry.file_name().unwrap().to_str().unwrap();
    let ignore_dir = dirs.iter().position(|&v| v == name) != None;

    if ignore_dir {
      continue;
    }

    let ext = entry.extension();

    if ext != None {
      let dir = map.get(ext.unwrap().to_str().unwrap());

      if dir != None {
        let dir = format!("{}/{}", path, dir.unwrap());
        utils::move_file(entry, dir);
      }
      else {
        utils::move_file(entry, globals::DIRS[6].to_string());
      }
    }
    else {
      utils::move_file(entry, globals::DIRS[6].to_string());
    }
  }
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
    utils::create_dirs(&DIR.to_string());

    let files = vec![
      "foo.txt", "foo.png", "foo.mp3", "foo.mp4", "foo.zip", "foo.exe",
    ];

    create_files(&files);
    organize(DIR.to_string());

    let expect = vec![
      "Text/foo.txt",
      "Image/foo.png",
      "Audio/foo.mp3",
      "Video/foo.mp4",
      "Compressed/foo.zip",
      "Executable/foo.exe",
    ];
    assert_all(expect);

    fs::remove_dir_all(DIR).expect("unable to remove");
  }

  fn create_files(files: &Vec<&str>) {
    for f in files {
      let path = format!("{}/{}", DIR, f);
      File::create(path).expect("unable to create file");
    }
  }

  fn assert_all(files: Vec<&str>) {
    for f in files {
      let path = format!("{}/{}", DIR, f);
      assert!(Path::new(&path).exists());
    }
  }
}
