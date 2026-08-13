use crate::error::AnyError;
use crate::globals;
use directories::UserDirs;
use std::path::Path;

/// Resolve the directory to organize.
///
/// When no explicit `dir` was supplied, prefer the OS-provided Downloads
/// directory if it exists, falling back to `<home>/Downloads`. An explicit
/// `dir` is canonicalised (and must exist).
pub fn get_download_dir(dir: Option<&Path>) -> Result<String, AnyError> {
  match dir {
    None => {
      let user = UserDirs::new().ok_or(AnyError::DownloadDirNotFound)?;
      resolve_from_dir(user.download_dir(), user.home_dir())
    },
    Some(path) => {
      let canonical =
        path
          .canonicalize()
          .map_err(|source| AnyError::InvalidPath {
            path: path.to_path_buf(),
            source,
          })?;
      Ok(canonical.to_string_lossy().into_owned())
    },
  }
}

/// Resolve the download directory, preferring the OS-provided one when it
/// exists and falling back to `<home>/Downloads`.
///
/// Split out from [`get_download_dir`] so it can be tested deterministically
/// without depending on the real user's home/download directories.
fn resolve_from_dir(
  download: Option<&Path>,
  home: &Path,
) -> Result<String, AnyError> {
  let fallback = home.join(globals::DIR);
  let download = download
    .filter(|d| d.exists())
    .map_or_else(|| fallback.clone(), Path::to_path_buf);

  if download.exists() {
    Ok(download.to_string_lossy().into_owned())
  } else {
    Err(AnyError::DownloadDirNotFound)
  }
}

#[cfg(test)]
mod test {
  #![allow(clippy::expect_used, clippy::unwrap_used, clippy::indexing_slicing)]
  use super::*;
  use crate::test_utils::TempDir;

  #[test]
  fn no_arguments_prefers_existing_download_dir() {
    let home = TempDir::new("home");
    let dl = home.path().join("Downloads");
    std::fs::create_dir(&dl).expect("create fake Downloads");

    let dir = resolve_from_dir(Some(&dl), home.path()).expect("should resolve");
    assert!(Path::new(&dir).exists());
    assert!(dir.ends_with("Downloads"));
  }

  #[test]
  fn no_arguments_falls_back_to_home_downloads() {
    let home = TempDir::new("home2");
    let dl = home.path().join("Downloads");
    std::fs::create_dir(&dl).expect("create fake Downloads");

    // OS-provided dir does not exist -> fall back to <home>/Downloads.
    let missing = home.path().join("nope");
    let dir =
      resolve_from_dir(Some(&missing), home.path()).expect("should fallback");
    assert_eq!(Path::new(&dir), dl.as_path());
  }

  #[test]
  fn no_arguments_errors_when_download_missing() {
    let home = TempDir::new("home3");
    // No Downloads dir anywhere.
    assert!(
      matches!(
        resolve_from_dir(None, home.path()),
        Err(AnyError::DownloadDirNotFound)
      ),
      "expected DownloadDirNotFound"
    );
  }

  #[test]
  fn with_one_argument_canonicalizes() {
    let temp = TempDir::new("canon");
    let arg = temp.path().to_str().expect("temp path is utf-8");
    let dir =
      get_download_dir(Some(Path::new(arg))).expect("should canonicalize");
    // Compare against the canonicalized temp path: `fs::canonicalize` returns
    // a verbatim `\\?\...` path on Windows (and an absolute path on Unix), so
    // canonicalizing the expectation makes the equality check platform-stable.
    let expected = temp.path().canonicalize().expect("canonicalize temp");
    assert_eq!(Path::new(&dir), expected);
  }

  #[test]
  fn with_one_argument_errors_on_missing_path() {
    assert!(matches!(
      get_download_dir(Some(Path::new("./definitely-not-a-real-dir-xyz"))),
      Err(AnyError::InvalidPath { .. })
    ));
  }
}
