use std::env;

mod globals;
mod utils;

fn main() {
  let dir = utils::get_download_dir(env::args().collect());
  utils::create_dirs(&dir);
  utils::organize(&dir);
}
