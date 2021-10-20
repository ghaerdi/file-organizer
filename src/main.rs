use std::env;

mod globals;
mod utils;

fn main() {
  let dir = utils::get_download_dir(env::args().collect());
  utils::set_write_permissions(&dir);
  utils::create_dirs(&dir);
  utils::organize(dir);
}
