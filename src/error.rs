use std::io;
use std::path::PathBuf;

/// Errors that can occur while organizing a directory.
#[derive(Debug, thiserror::Error)]
pub enum AnyError {
  #[error("unable to open directory")]
  OpenDir(#[source] io::Error),

  #[error("unable to read directory entry")]
  ReadEntry(#[source] io::Error),

  #[error("unable to get metadata for {path}")]
  GetMetadata {
    path: PathBuf,
    #[source]
    source: io::Error,
  },

  #[error("unable to set permissions on {path}")]
  SetPermissions {
    path: PathBuf,
    #[source]
    source: io::Error,
  },

  #[error("unable to move file to {to}")]
  MoveFile {
    to: String,
    #[source]
    source: io::Error,
  },

  #[error("unable to determine file name for {path}")]
  FileName { path: PathBuf },

  #[error("unable to create {name} directory")]
  CreateDir {
    name: String,
    #[source]
    source: io::Error,
  },

  #[error("no Other category configured")]
  NoOtherCategory,

  #[error("Downloads directory not found")]
  DownloadDirNotFound,

  #[error("too many arguments")]
  TooManyArguments,

  #[error("unable to resolve path {path}")]
  InvalidPath {
    path: PathBuf,
    #[source]
    source: io::Error,
  },
}
