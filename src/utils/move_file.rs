use std::error::Error;
use std::fs;
use std::path::Path;

pub fn move_file(from: &Path, to: &str) -> Result<(), Box<dyn Error>> {
  let file_name = from
    .file_name()
    .and_then(|n| n.to_str())
    .ok_or("unable to determine file name")?;
  let to = format!("{to}/{file_name}");

  fs::rename(from, &to).map_err(|_| "unable to move file")?;
  println!("{}", from.to_string_lossy());
  println!("moved to");
  println!("{to}");
  println!();
  Ok(())
}
