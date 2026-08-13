use crate::globals;
use directories::UserDirs;
use std::error::Error;
use std::path::Path;

static DOWNLOAD_DIR_NOT_FOUND: &str = "Downloads directory not found";

pub fn get_download_dir(args: &[&str]) -> Result<String, Box<dyn Error>> {
  match args.len() {
    1 => {
      let user = UserDirs::new().ok_or(DOWNLOAD_DIR_NOT_FOUND)?;
      let fallback = format!("{}/{}", user.home_dir().display(), globals::DIR);
      let download = user
        .download_dir()
        .filter(|d| d.exists())
        .map_or(fallback, |d| d.to_string_lossy().into_owned());

      if Path::new(&download).exists() {
        Ok(download)
      } else {
        Err(DOWNLOAD_DIR_NOT_FOUND.into())
      }
    },
    2 => {
      let canonical =
        Path::new(args.get(1).ok_or(DOWNLOAD_DIR_NOT_FOUND)?).canonicalize();
      canonical.map_or_else(
        |_| Err(DOWNLOAD_DIR_NOT_FOUND.into()),
        |dir| Ok(dir.to_string_lossy().into_owned()),
      )
    },
    _ => Err("too many arguments".into()),
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn without_arguments() {
    let args = vec![""];
    match get_download_dir(&args) {
      Ok(dir) => assert!(dir.contains("Downloads")),
      Err(_) => {},
    }
  }

  #[test]
  fn with_one_argument() {
    let args = vec!["", "Downloads"];
    match get_download_dir(&args) {
      Ok(dir) => assert!(dir.contains("Downloads")),
      Err(_) => {},
    }
  }

  #[test]
  fn too_many_arguments_is_error() {
    let args = vec!["", "Foo", "Bar"];
    assert!(get_download_dir(&args).is_err());
  }
}
