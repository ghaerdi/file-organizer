use std::error::Error;
use std::fs;
use std::path::Path;

// On Unix, `set_readonly(false)` sets the permissions to `0o666` (read/write for all),
// which is not ideal. However, for the purpose of this script, which is to organize the user's "Downloads" folder,
// simply making the files writable is probably sufficient and less likely to cause issues.
#[allow(clippy::permissions_set_readonly_false)]
pub fn set_write_permissions<P: AsRef<Path>>(
  path: P,
) -> Result<(), Box<dyn Error>> {
  let dir = fs::read_dir(path).map_err(|_| "unable to open directory")?;

  for entry in dir {
    let entry = entry.map_err(|_| "unable to read directory entry")?.path();
    let mt = fs::metadata(&entry).map_err(|_| "unable to get metadata")?;
    let mut perms = mt.permissions();
    perms.set_readonly(false);
    fs::set_permissions(entry, perms)
      .map_err(|_| "unable to set permissions")?;
  }
  Ok(())
}
