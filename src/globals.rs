pub static DIR: &str = "Downloads";
pub static DIRS: [&str; 7] = [
  "Text",
  "Image",
  "Video",
  "Audio",
  "Compressed",
  "Executable",
  "Other",
];
pub static TEXT_EXT: [&str; 8] =
  ["md", "txt", "doc", "docx", "pptx", "odf", "docm", "pdf"];
pub static IMAGE_EXT: [&str; 9] = [
  "jpg", "png", "jpeg", "gif", "tiff", "psd", "bmp", "ico", "svg",
];
pub static AUDIO_EXT: [&str; 4] = ["mp3", "wma", "wav", "flac"];
pub static VIDEO_EXT: [&str; 6] = ["mov", "mp4", "avi", "mkv", "flv", "wmv"];
pub static COMPRESSED_EXT: [&str; 6] =
  ["zip", "rar", "rar5", "7z", "ace", "gz"];
pub static EXECUTABLE_EXT: [&str; 4] = ["exe", "msi", "deb", "rpm"];

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn all_arr_values_starts_with_uppercase() {
    let result: bool = {
      let mut b = true;

      for value in DIRS {
        if value[..1] != value[..1].to_uppercase() {
          b = false;
          break;
        }
      }

      b
    };

    assert!(result);
  }

  #[test]
  fn no_dots_in_files_extension() {
    assert!(!start_with_dot(TEXT_EXT.to_vec()));
    assert!(!start_with_dot(IMAGE_EXT.to_vec()));
    assert!(!start_with_dot(AUDIO_EXT.to_vec()));
    assert!(!start_with_dot(VIDEO_EXT.to_vec()));
    assert!(!start_with_dot(COMPRESSED_EXT.to_vec()));
    assert!(!start_with_dot(EXECUTABLE_EXT.to_vec()));
  }

  fn start_with_dot(arr: Vec<&'static str>) -> bool {
    for s in arr {
      if s.starts_with('.') {
        return true;
      }
    }
    false
  }
}
