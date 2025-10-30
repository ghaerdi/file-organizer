use std::fs;
use std::path::PathBuf;

pub fn move_file(from: PathBuf, to: String) {
  let file_name = from.file_name().unwrap();
  let file_name = file_name.to_os_string();
  let file_name = file_name.to_str().unwrap();
  let to = format!("{to}/{file_name}");

  fs::rename(&from, &to).expect("unable to move");
  println!("{}", from.to_str().unwrap());
  println!("moved to");
  println!("{to}");
  println!();
}
