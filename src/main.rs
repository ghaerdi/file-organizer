use std::env;

mod globals;
mod utils;

fn main() {
  let args: Vec<String> = env::args().collect();
  let args: Vec<&str> = args.iter().map(|v| v.as_str()).collect();
  let dir = utils::get_download_dir(&args);
  utils::set_write_permissions(&dir);
  utils::create_dirs(&dir);
  utils::organize(&dir);
}
