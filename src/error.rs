use std::io;
use std::path::PathBuf;

/// Errors that can occur while organizing a directory.
#[derive(Debug, thiserror::Error)]
pub enum AnyError {
  #[error("could not open the target directory")]
  OpenDir(#[source] io::Error),

  #[error("could not read an entry in the target directory")]
  ReadEntry(#[source] io::Error),

  #[error("could not read the metadata of {path}")]
  GetMetadata {
    path: PathBuf,
    #[source]
    source: io::Error,
  },

  #[error("could not change the permissions of {path}")]
  SetPermissions {
    path: PathBuf,
    #[source]
    source: io::Error,
  },

  #[error("could not move the file to {to}")]
  MoveFile {
    to: String,
    #[source]
    source: io::Error,
  },

  #[error("could not determine the file name of {path}")]
  FileName { path: PathBuf },

  #[error("could not create the {name} directory")]
  CreateDir {
    name: String,
    #[source]
    source: io::Error,
  },

  #[error("no Other category is configured")]
  NoOtherCategory,

  #[error("the Downloads directory could not be found")]
  DownloadDirNotFound,

  #[error("could not resolve the path {path}")]
  InvalidPath {
    path: PathBuf,
    #[source]
    source: io::Error,
  },

  #[error("could not determine the location of the move register")]
  NoRegisterDir,

  #[error("could not write the move register")]
  RegisterWrite(#[source] io::Error),

  #[error("could not read the move register: {0}")]
  RegisterRead(String),

  #[error("could not parse the move register")]
  RegisterParse(String),

  #[error("no run recorded for this directory to revert")]
  NothingToRevert,

  #[error("could not create the {name} directory while reverting")]
  RevertCreateDir {
    name: String,
    #[source]
    source: io::Error,
  },

  #[error("could not move the file back to {to} while reverting")]
  RevertMoveFile {
    to: String,
    #[source]
    source: io::Error,
  },

  #[error("could not remove the move register entry")]
  RegisterPrune(#[source] io::Error),
}
