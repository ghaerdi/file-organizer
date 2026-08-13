use crate::globals::{
  AUDIO_EXT, CODE_CONFIG_EXT, CODE_EXT, COMPRESSED_EXT, DATA_EXT, DIRS,
  DISK_EXT, EXECUTABLE_EXT, FONT_EXT, IMAGE_EXT, TEXT_EXT, VIDEO_EXT,
};
use std::collections::HashMap;

pub fn files_extension() -> HashMap<&'static str, &'static str> {
  let mut map = HashMap::new();

  to_map(&mut map, &TEXT_EXT, DIRS[0]);
  to_map(&mut map, &IMAGE_EXT, DIRS[1]);
  to_map(&mut map, &VIDEO_EXT, DIRS[2]);
  to_map(&mut map, &AUDIO_EXT, DIRS[3]);
  to_map(&mut map, &COMPRESSED_EXT, DIRS[4]);
  to_map(&mut map, &EXECUTABLE_EXT, DIRS[5]);
  to_map(&mut map, &CODE_EXT, DIRS[6]);
  to_map(&mut map, &CODE_CONFIG_EXT, DIRS[6]);
  to_map(&mut map, &DATA_EXT, DIRS[7]);
  to_map(&mut map, &DISK_EXT, DIRS[8]);
  to_map(&mut map, &FONT_EXT, DIRS[9]);
  map
}

fn to_map(
  map: &mut HashMap<&'static str, &'static str>,
  ext: &[&'static str],
  dir: &'static str,
) {
  for ext in ext {
    map.insert(ext, dir);
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::collections::HashMap;

  #[test]
  fn map_frontend_frameworks() {
    let mut map = HashMap::new();
    to_map(&mut map, &["Vue", "Svelte"], "Frontend");

    let mut expect = HashMap::new();
    expect.insert("Vue", "Frontend");
    expect.insert("Svelte", "Frontend");

    assert_eq!(map, expect);
  }

  #[test]
  fn files_ext() {
    let map = files_extension();

    assert_eq!(map.get("txt"), Some(&"Text"));
    assert_eq!(map.get("png"), Some(&"Image"));
    assert_eq!(map.get("mp3"), Some(&"Audio"));
    assert_eq!(map.get("mp4"), Some(&"Video"));
    assert_eq!(map.get("rar"), Some(&"Compressed"));
    assert_eq!(map.get("exe"), Some(&"Executable"));
    assert_eq!(map.get("rs"), Some(&"Code"));
    assert_eq!(map.get("toml"), Some(&"Code"));
    assert_eq!(map.get("csv"), Some(&"Data"));
    assert_eq!(map.get("xlsx"), Some(&"Data"));
    assert_eq!(map.get("iso"), Some(&"Disk"));
    assert_eq!(map.get("dmg"), Some(&"Disk"));
    assert_eq!(map.get("ttf"), Some(&"Font"));
    assert_eq!(map.get("woff2"), Some(&"Font"));
  }
}
