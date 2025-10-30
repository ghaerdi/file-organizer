use std::fs;
use std::path::Path;

// On Unix, `set_readonly(false)` sets the permissions to `0o666` (read/write for all),
// which is not ideal. However, for the purpose of this script, which is to organize the user's "Downloads" folder,
// simply making the files writable is probably sufficient and less likely to cause issues.
#[allow(clippy::permissions_set_readonly_false)]
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
