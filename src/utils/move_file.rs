use std::path::PathBuf;
use std::fs;
use crate::globals;

pub fn move_file(from: PathBuf, to: &str) {
  let file_name = from.file_name().unwrap();
  let file_name = file_name.to_os_string();
  let file_name = file_name.to_str().unwrap();
  let to = format!("{}/{}/{}", globals::PATH, to, file_name);

  fs::rename(&from, &to).expect("unable to move");
  println!("{} moved to: {}", from.to_str().unwrap(), to);
}

