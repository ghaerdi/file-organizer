use crate::globals::{
  AUDIO_EXT, COMPRESSED_EXT, DIRS, EXECUTABLE_EXT, IMAGE_EXT, TEXT_EXT,
  VIDEO_EXT,
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

  return map;
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
        let mut map= HashMap::new();
        to_map(&mut map, &["Vue", "Svelte"], "Frontend");

        let mut expect = HashMap::new();
        expect.insert("Vue", "Frontend");
        expect.insert("Svelte", "Frontend");

        assert_eq!(map, expect);
    }

    #[test]
    fn files_ext() {
        let map = files_extension();

        assert_eq!(map.get("txt").unwrap_or(&""), &"Text");
        assert_eq!(map.get("png").unwrap_or(&""), &"Image");
        assert_eq!(map.get("mp3").unwrap_or(&""), &"Audio");
        assert_eq!(map.get("mp4").unwrap_or(&""), &"Video");
        assert_eq!(map.get("rar").unwrap_or(&""), &"Compressed");
        assert_eq!(map.get("exe").unwrap_or(&""), &"Executable");
    }
}
