import os
import shutil

# Variables
download_dir_path: str = "~/Downloads"
download_dir_path = os.path.expanduser(download_dir_path)

files_extensions: dict = {
    "Image": ['.jpg', '.png', 'jpeg', '.gif', '.tiff', '.psd', '.bmp', '.ico', '.svg'],
    "Text": ['.txt', '.doc', '.docx', 'pptx', '.odf', '.docm', '.pdf'],
    "Video": ['.mov', '.mp4', '.avi', '.mkv', '.mkv', '.flv', '.wmv'],
    "Audio": ['.mp3', '.wma', '.wav', '.flac'],
    "Other": []
}

# Lambda Functions
scan_files = lambda dir_path: [file.name for file in os.scandir(dir_path) if file.is_file()]
scan_directories = lambda dir_path: [directory.name for directory in os.scandir(dir_path) if directory.is_dir()]

# Functions
def organize_files(dir_path, directory_name, files_extension):
    for file in scan_files(dir_path):
        for ext in files_extension:
            if (file.find(ext) != -1):
                create_directory(dir_path, directory_name)
                shutil.move(f"{dir_path}/{file}", f"{dir_path}/{directory_name}")
                print(f"{file} -> {directory_name}")

def create_directory(dir_path, desired_directory):
    if not desired_directory in scan_directories(dir_path):
        os.mkdir(f"{dir_path}/{desired_directory}")
        print(f"new directory: {desired_directory}")

def organize_directory(dir_path, files_ext):
    for directory_name in files_ext:
        if (directory_name != "Other"):
            organize_files(dir_path, directory_name, files_ext.get(directory_name))
        else:
            for file in scan_files(dir_path):
                create_directory(dir_path, directory_name)
                shutil.move(f"{dir_path}/{file}", f"{dir_path}/{directory_name}")
                print(f"{file} -> {directory_name}")

organize_directory(download_dir_path, files_extensions)
