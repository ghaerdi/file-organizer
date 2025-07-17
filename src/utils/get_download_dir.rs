use crate::globals;
use directories::UserDirs;
use std::path::Path;

static DOWNLOAD_DIR_NOT_FOUND: &str = "Downloads directory not found";

pub fn get_download_dir(args: &[&str]) -> String {
  match args.len() {
    1 => {
      let user = UserDirs::new().unwrap();
      let download = user.download_dir();

      match download {
        None => {
          let home = user.home_dir();
          let download = format!("{}/{}", home.display(), globals::DIR);
          let download = Path::new(&download);

          if !download.exists() {
            panic!("{}", DOWNLOAD_DIR_NOT_FOUND);
          }

          download.to_str().unwrap().to_string()
        },
        Some(dir) => {
          if !dir.exists() {
            panic!("{}", DOWNLOAD_DIR_NOT_FOUND);
          }

          dir.to_str().unwrap().to_string()
        },
      }
    },
    2 => {
      let download = Path::new(&args[1]);
      let download = match download.canonicalize() {
        Ok(dir) => dir,
        Err(_) => panic!("{}", DOWNLOAD_DIR_NOT_FOUND),
      };

      download.to_str().unwrap().to_string()
    },
    _ => {
      panic!("to mutch arguments");
    },
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::panic::catch_unwind;

  #[test]
  fn without_arguments() {
    let args = vec![""];
    let result = catch_unwind(|| get_download_dir(&args));

    match result {
      Ok(dir) => assert!(dir.contains("Downloads")),
      Err(_) => assert!(result.is_err()),
    }
  }

  #[test]
  fn with_one_argument() {
    let args = vec!["", "Downloads"];
    let result = catch_unwind(|| get_download_dir(&args));

    match result {
      Ok(dir) => assert!(dir.contains("Downloads")),
      Err(_) => assert!(result.is_err()),
    }
  }

  #[test]
  #[should_panic]
  fn pass_more_of_one_argument() {
    let args = vec!["", "Foo", "Bar"];
    get_download_dir(&args);
  }
}
