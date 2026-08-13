pub static DIR: &str = "Downloads";
pub static DIRS: [&str; 11] = [
  "Text",
  "Image",
  "Video",
  "Audio",
  "Compressed",
  "Executable",
  "Code",
  "Data",
  "Disk",
  "Font",
  "Other",
];
pub static TEXT_EXT: [&str; 12] = [
  "md", "txt", "doc", "docx", "pptx", "odt", "odf", "docm", "pdf", "rtf",
  "tex", "org",
];
pub static IMAGE_EXT: [&str; 17] = [
  "jpg", "png", "jpeg", "gif", "tiff", "psd", "bmp", "ico", "svg", "webp",
  "heic", "heif", "avif", "raw", "cr2", "nef", "arw",
];
pub static VIDEO_EXT: [&str; 15] = [
  "mov", "mp4", "avi", "mkv", "flv", "wmv", "webm", "m4v", "mpg", "mpeg",
  "3gp", "3g2", "ts", "vob", "ogv",
];
pub static AUDIO_EXT: [&str; 13] = [
  "mp3", "wma", "wav", "flac", "aac", "ogg", "opus", "m4a", "oga", "mid",
  "midi", "aiff", "ape",
];
pub static COMPRESSED_EXT: [&str; 15] = [
  "zip", "rar", "rar5", "7z", "ace", "gz", "tar", "tgz", "bz2", "xz", "lzma",
  "lz", "zst", "cab", "cbz",
];
pub static EXECUTABLE_EXT: [&str; 10] = [
  "exe", "msi", "deb", "rpm", "appimage", "apk", "com", "bat", "cmd", "run",
];
pub static CODE_EXT: [&str; 28] = [
  "rs", "ts", "js", "jsx", "tsx", "py", "go", "java", "c", "cpp", "h", "hpp",
  "cs", "php", "rb", "sh", "lua", "swift", "kt", "kts", "scala", "dart", "r",
  "pl", "sql", "html", "css", "json",
];
pub static CODE_CONFIG_EXT: [&str; 10] = [
  "yaml", "yml", "toml", "xml", "ini", "cfg", "conf", "env", "lock", "gradle",
];
pub static DATA_EXT: [&str; 14] = [
  "csv", "xls", "xlsx", "ods", "db", "sqlite", "sqlite3", "db3", "parquet",
  "jsonl", "tsv", "sav", "dta", "feather",
];
pub static DISK_EXT: [&str; 7] =
  ["iso", "img", "dmg", "vhd", "vdi", "vmdk", "qcow2"];
pub static FONT_EXT: [&str; 5] = ["ttf", "otf", "woff", "woff2", "eot"];

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
    assert!(!start_with_dot(CODE_EXT.to_vec()));
    assert!(!start_with_dot(CODE_CONFIG_EXT.to_vec()));
    assert!(!start_with_dot(DATA_EXT.to_vec()));
    assert!(!start_with_dot(DISK_EXT.to_vec()));
    assert!(!start_with_dot(FONT_EXT.to_vec()));
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
