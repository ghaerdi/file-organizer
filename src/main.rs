mod globals;
mod utils;

fn main() {
  let dir = utils::get_download_dir();
  utils::create_dirs(&dir);
  utils::organize(&dir);
}
