use std::env;
use std::error::Error;

mod globals;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();
  let args: Vec<&str> = args.iter().map(String::as_str).collect();
  let dir = utils::get_download_dir(&args)?;
  utils::set_write_permissions(&dir)?;
  utils::create_dirs(&dir)?;
  utils::organize(&dir)?;
  Ok(())
}
