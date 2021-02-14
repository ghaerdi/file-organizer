#!/usr/bin/env python3
import os
import sys
import shutil
import pathlib

# Functions
scan_files: [str] = lambda dir_path: [file.name for file in os.scandir(dir_path) if file.is_file()]

def handle_sys_arguments(system_arguments: [str]) -> str:
    if (len(sys.argv) > 2):
        print("You don't need pass more than 1 commands")
    if (len(sys.argv) > 1):
        return sys.argv[1]

def organize_files(dir_path: str, directory_name: str, files_extension: [str]):
    for file in scan_files(dir_path):
        for ext in files_extension:
            if (file.find(ext) != -1):
               move_file(dir_path, directory_name, file) 

def move_file(dir_path: str, directory_name: str, file: str):
    file_path: str = f"{dir_path}/{file}"
    desired_directory_path: str = f"{dir_path}/{directory_name}"
    create_directory(dir_path, directory_name)
    shutil.move(file_path, desired_directory_path)
    print(f"{file} -> {directory_name}")

def create_directory(dir_path: str, desired_directory: str):
    desired_directory_path: str = f"{dir_path}/{desired_directory}"
    if not os.path.exists(desired_directory_path):
        os.mkdir(desired_directory_path)
        print(f"New directory: {desired_directory}")

def organize_directory(dir_path: str, files_ext: object):
    for directory_name in files_ext:
        if (directory_name != "Other"):
            organize_files(dir_path, directory_name, files_ext.get(directory_name))
        else:
            for file in scan_files(dir_path):
                move_file(dir_path, directory_name, file)

# Variables
download_dir_path: str = handle_sys_arguments(sys.argv) or os.path.expanduser("~/Downloads")
download_dir_path = pathlib.Path(download_dir_path).absolute() 
files_extensions: dict = {
    "Image": ['.jpg', '.png', 'jpeg', '.gif', '.tiff', '.psd', '.bmp', '.ico', '.svg'],
    "Text": ['.txt', '.doc', '.docx', 'pptx', '.odf', '.docm', '.pdf'],
    "Video": ['.mov', '.mp4', '.avi', '.mkv', '.mkv', '.flv', '.wmv'],
    "Audio": ['.mp3', '.wma', '.wav', '.flac'],
    "Other": []
}


organize_directory(download_dir_path, files_extensions)
