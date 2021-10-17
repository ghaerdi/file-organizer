use std::fs;
use crate::utils;
use crate::globals;

pub fn organize() {
  let dir = fs::read_dir(globals::PATH).expect("unable to open");
  let map = utils::files_extension();
  let dirs = globals::DIRS.to_vec();

  for entry in dir {
    let path = entry.unwrap().path();
    let name = path.file_name().unwrap().to_str().unwrap();
    let ignore_dir = dirs.iter().position(|&v| v == name) != None;

    if ignore_dir {
        continue;
    }

    let ext = path.extension();

    if ext != None {
        let dir = map.get(ext.unwrap().to_str().unwrap());

        if dir != None {
            utils::move_file(path, dir.unwrap());
        }
        else {
            utils::move_file(path, globals::DIRS[6]);
        }
    }
    else {
        utils::move_file(path, globals::DIRS[6]);
    }
  }
}

