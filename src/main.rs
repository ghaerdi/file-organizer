#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::fs::{read_dir, ReadDir, rename, metadata};
use std::path::PathBuf;
// use std::io::BufReader;

const PATH: &str = "./download";

#[derive(Debug)]
struct FilesExtension {
	image: Vec<String>,
	text: Vec<String>,
	video: Vec<String>,
	audio: Vec<String>,
	compressed: Vec<String>,
	executable: Vec<String>,
}


impl FilesExtension {
	fn new() -> Self {
		return FilesExtension {
			image: convert(vec![
				".jpg", ".png", "jpeg", ".gif", ".tiff", ".psd", ".bmp", ".ico", ".svg",
			]),
			text: convert(vec![
				".txt", ".doc", ".docx", "pptx", ".odf", ".docm", ".pdf",
			]),
			video: convert(vec![".mov", ".mp4", ".avi", ".mkv", ".mkv", ".flv", ".wmv"]),
			audio: convert(vec![".mp3", ".wma", ".wav", ".flac"]),
			compressed: convert(vec![".zip", ".rar", ".rar5", ".7z", ".ace", ".gz"]),
			executable: convert(vec![".exe", ".msi", ".deb", ".rpm"]),
		};
	}
}

fn convert(vec: Vec<&str>) -> Vec<String> {
	return vec.iter().map(|s| s.to_string()).collect();
}

fn get_directory(path: &str) -> ReadDir {
	return read_dir(path).expect("unable to open");
}

fn move_file(from: PathBuf, to: &str) {
	let from = from.into_os_string();
	let from = from.into_string().expect("unable to get name");
	let file_name = &from[2..];
	let to = format!("{}/{}", to, file_name);
	let md = metadata(&from).unwrap();
	if md.is_file() {
		println!("{:?} {:?}", from, to);
		// rename(from, to).expect("unable to move");
		std::fs::copy(from, to).expect("unable to copy");
	}
}

fn main() {
	let dir = get_directory("./");

	for entry in dir {
		// println!("{:?}", entry.unwrap().path());
		move_file(entry.unwrap().path(), "./download");
	}
}
