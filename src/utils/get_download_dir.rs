use crate::error::AnyError;
use crate::globals;
use directories::UserDirs;
use std::path::Path;

pub fn get_download_dir(args: &[&str]) -> Result<String, AnyError> {
  match args.len() {
    1 => {
      let user = UserDirs::new().ok_or(AnyError::DownloadDirNotFound)?;
      let fallback = format!("{}/{}", user.home_dir().display(), globals::DIR);
      let download = user
        .download_dir()
        .filter(|d| d.exists())
        .map_or(fallback, |d| d.to_string_lossy().into_owned());

      if Path::new(&download).exists() {
        Ok(download)
      } else {
        Err(AnyError::DownloadDirNotFound)
      }
    },
    2 => {
      let path = Path::new(args.get(1).ok_or(AnyError::DownloadDirNotFound)?);
      let canonical =
        path
          .canonicalize()
          .map_err(|source| AnyError::InvalidPath {
            path: path.to_path_buf(),
            source,
          })?;
      Ok(canonical.to_string_lossy().into_owned())
    },
    _ => Err(AnyError::TooManyArguments),
  }
}

#[cfg(test)]
mod test {
  #![allow(clippy::expect_used, clippy::unwrap_used, clippy::indexing_slicing)]
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
