/*
 * Vento, a CLI inventory for your files.
 * Copyright (C) 2022 Lux Aliaga
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use std::fs;
use fs_extra::dir::{CopyOptions, move_dir};
use std::path::{Path, PathBuf};
use colored::Colorize;
use super::common;

pub fn take(file: &String) { // Takes a file or directory
    let ventodir = common::env_config();
    let active: PathBuf = [ventodir.to_path_buf(), Path::new("active").to_path_buf()].iter().collect();
    
    let sourcepath: PathBuf = Path::new(&file).to_path_buf();
    let destpath: PathBuf = [&active, &Path::new(file).to_path_buf()].iter().collect();
    
    if Path::exists(&destpath) {
        println!("❌ {}", format!("A file with the same name already exists in your inventory!").red());
    } else if sourcepath.is_file() | sourcepath.is_symlink() {
        fs::copy(&file, &destpath).expect("❌ Vento was unable to copy the file.");
        fs::remove_file(&file).expect("❌ Vento was unable to remove the file.");
    } else if sourcepath.is_dir() {
        let options = CopyOptions::new();
        move_dir(&file, &active, &options).expect("❌ Vento was unable to move the directory.");
    } else {
        println!("❌ {}", format!("No such file or directory.").red());
    }
}

pub fn drop(file: &String, dest: PathBuf) { // Drops a file or directory
    let ventodir = common::env_config();
    // I know I've declared the same variable multiple times through this project. I might move this to the common file and just call it from there every single time I need it, but that'll be for later.
    let active: PathBuf = [ventodir.to_path_buf(), Path::new("active").to_path_buf()].iter().collect(); 
    
    let sourcepath: PathBuf = [&active, &Path::new(file).to_path_buf()].iter().collect();
    let destpath: PathBuf = [Path::new(&dest).to_path_buf(), Path::new(file).to_path_buf()].iter().collect();

    if Path::exists(&destpath) { // HAHA YANDEREDEV MOMENT. This checks what method to use for the file/directory the user has picked
        println!("❌ {}", format!("A file with the same name already exists in the destination! Try renaming it or dropping this file somewhere else.").red());
    } else if sourcepath.is_file() | sourcepath.is_symlink() {
        fs::copy(&sourcepath, &destpath).expect("❌ Vento was unable to copy the file.");
        fs::remove_file(&sourcepath).expect("❌ Vento was unable to remove the file.");
    } else if sourcepath.is_dir() {
        let destpath: PathBuf = Path::new(&dest).to_path_buf();
        let options = CopyOptions::new();
        move_dir(&sourcepath, &destpath, &options).expect("❌ Vento was unable to move the directory.");
    } else {
        println!("❌ {}", format!("No such file or directory.").red());
    }
}
