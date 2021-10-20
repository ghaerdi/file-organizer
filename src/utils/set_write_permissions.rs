use std::fs;
use std::path::Path;

pub fn set_write_permissions<P: AsRef<Path>>(path: P) {
  let dir = fs::read_dir(path).expect("unable to open");

  for entry in dir {
    let entry = entry.unwrap().path();
    let mt = fs::metadata(&entry).expect("unable to get metadata");
    let mut perms = mt.permissions();
    perms.set_readonly(false);
    fs::set_permissions(entry, perms).expect("unable to set permmissions");
  }
}
