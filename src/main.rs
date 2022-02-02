use std::env;

mod globals;
mod utils;

fn main() {
  let args = env::args().collect();
  let dir = utils::get_download_dir(args);
  utils::set_write_permissions(&dir);
  utils::create_dirs(&dir);
  utils::organize(dir);
}
