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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::path::Path;

    // create a dir and then call the create_dirs function, check if exist each directory and then
    // remove all the dirs.
    #[test]
    fn create_directories() {
        let main_dir = "./create_dirs";
        fs::create_dir(&main_dir).expect("unable to create");
        create_dirs(&main_dir.to_string());

        assert!(Path::new("./create_dirs/Text").exists());
        assert!(Path::new("./create_dirs/Image").exists());
        assert!(Path::new("./create_dirs/Audio").exists());
        assert!(Path::new("./create_dirs/Video").exists());
        assert!(Path::new("./create_dirs/Compressed").exists());
        assert!(Path::new("./create_dirs/Executable").exists());

        fs::remove_dir_all("./create_dirs").expect("unable to remove");
    }
}
