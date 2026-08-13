use crate::error::AnyError;
use std::fs;
use std::path::Path;

pub fn move_file(from: &Path, to: &str) -> Result<(), AnyError> {
  let file_name =
    from.file_name().and_then(|n| n.to_str()).ok_or_else(|| {
      AnyError::FileName {
        path: from.to_path_buf(),
      }
    })?;
  let to = format!("{to}/{file_name}");

  fs::rename(from, &to).map_err(|source| AnyError::MoveFile {
    to: to.clone(),
    source,
  })?;
  println!("moved to");
  println!("{to}");
  println!();
  Ok(())
}
