use crate::globals;
use std::fs;
use std::path::Path;

pub fn create_dirs(dir: &String) {
  for value in globals::DIRS {
    let path = format!("{}/{}", dir, value);
    let path = Path::new(&path);

    if !path.exists() {
      let error_msg = format!("unable to create {} directory", value);
      fs::create_dir(path).expect(error_msg.as_str());
      println!("{} directory created", value);
    }
  }
}
