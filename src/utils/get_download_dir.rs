use crate::globals;
use directories::UserDirs;
use std::path::Path;

pub fn get_download_dir(args: Vec<String>) -> String {
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
            panic!("not download dir found");
          }

          return download.to_str().unwrap().to_string();
        },
        Some(dir) => {
          if !dir.exists() {
            panic!("not download dir found");
          }

          return dir.to_str().unwrap().to_string();
        },
      }
    },
    2 => {
      let download = Path::new(&args[1]);
      let download = match download.canonicalize() {
        Ok(dir) => dir,
        Err(_) => panic!("directory not found"),
      };

      return download.to_str().unwrap().to_string();
    },
    _ => {
      panic!("to mutch arguments");
    },
  }
}

// #[cfg(test)]

// mod test {
//     use super::*;

//     #[test]
//     fn get_download_dir_without_arguments() {
//         assert_eq!(get_download_dir())
//     }
// }
