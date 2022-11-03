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

use super::common;
use anyhow::{bail, Result};
use colored::Colorize;
use fs_extra::dir::{move_dir, CopyOptions};
use std::fs;
use std::path::{Path, PathBuf};

/// Takes a file or directory and stores it in an inventory slot
pub fn take(file: &String, slot: &str, message: bool) -> Result<()> {
    let ventodir = &common::env_config()?.vento_dir;

    if !ventodir.is_dir() {
        // Detects if Vento hasn't been initialized and bails if so
        bail!(
            "{}",
            "Vento not initialized. Run \"vento -i\" to initialize Vento".red()
        );
    };
    let slotdir: PathBuf = match slot {
        "active" | "a" => common::env_config()?.active_dir,
        "inactive" | "i" => common::env_config()?.inactive_dir,
        _ => PathBuf::new(),
    };

    if !slotdir.is_dir() {
        // Detects if the slot provided exists
        bail!(
            "{}",
            format!(
                "No such slot. Valid slots are {} and {}",
                "active".green().bold(),
                "inactive".blue().bold()
            )
            .red()
        );
    };

    let sourcepath: PathBuf = Path::new(&file).to_path_buf();
    let mut sourcelocation: PathBuf = fs::canonicalize(&sourcepath)?;
    sourcelocation.pop();
    let filename = Path::new(&file).file_name().unwrap().to_str().unwrap();
    let destpath: PathBuf = [&slotdir, &Path::new(&filename).to_path_buf()]
        .iter()
        .collect();

    if Path::exists(&destpath) {
        // Checks if there's a file with the same name in the inventory.
        bail!(
            "{}",
            "A file with the same name already exists in your inventory!".red()
        );
    }

    if sourcepath.is_file() | sourcepath.is_symlink() {
        // Checks the path's file type
        fs::copy(&file, &destpath)?;
        fs::remove_file(&file)?;
    } else if sourcepath.is_dir() {
        let options = CopyOptions::new();
        move_dir(&file, &slotdir, &options)?;
    } else {
        bail!("{}", "No such file or directory".red());
    }

    common::history(common::HistoryData {
        path: sourcelocation.clone(),
        file: String::from(filename),
        slot: String::from(slot),
        action: common::Action::Take,
    })?;

    if message {
        println!(
            "✅ {} {} {} ",
            "Took".green(),
            &filename.bold(),
            format!("from {}", &sourcelocation.to_str().unwrap()).green()
        );
    }

    Ok(())
}

/// Drops a file or directory and stores it in an inventory slot
pub fn drop(file: &String, slot: &str, dest: PathBuf, message: bool) -> Result<()> {
    // Drops a file or directory
    let ventodir = &common::env_config()?.vento_dir;

    if !ventodir.is_dir() {
        // Detects if Vento hasn't been initialized and bails if so
        bail!(
            "{}",
            "Vento not initialized. Run \"vento -i\" to initialize Vento".red()
        );
    };

    let slotdir: PathBuf = match slot {
        "active" | "a" => common::env_config()?.active_dir,
        "inactive" | "i" => common::env_config()?.inactive_dir,
        _ => PathBuf::new(),
    };

    if !slotdir.is_dir() {
        // Detects if the slot provided exists
        bail!(
            "{}",
            format!(
                "No such slot. Valid slots are {} and {}",
                "active".green().bold(),
                "inactive".blue().bold()
            )
            .red()
        );
    };

    let sourcepath: PathBuf = [&slotdir, &Path::new(file).to_path_buf()].iter().collect();
    let mut destpath: PathBuf = [
        Path::new(&dest).to_path_buf(),
        Path::new(file).to_path_buf(),
    ]
    .iter()
    .collect();

    if Path::exists(&destpath) {
        // Checks if there's a file with the same name in the destination path.
        bail!("{}", "A file with the same name already exists in the destination! Try renaming it or dropping this file somewhere else".red());
    }

    if sourcepath.is_file() | sourcepath.is_symlink() {
        // Checks the path's file type
        fs::copy(&sourcepath, &destpath)?;
        fs::remove_file(&sourcepath)?;
    } else if sourcepath.is_dir() {
        let destpath: PathBuf = Path::new(&dest).to_path_buf();
        let options = CopyOptions::new();
        move_dir(&sourcepath, &destpath, &options)?;
    } else {
        bail!("{}", "No such file or directory".red());
    }

    destpath.pop();

    common::history(common::HistoryData {
        path: destpath.clone(),
        file: String::from(file),
        slot: String::from(slot),
        action: common::Action::Drop,
    })?;

    if message {
        println!(
            "✅ {} {} {} ",
            "Dropped".green(),
            &file.bold(),
            format!("into {}", &destpath.to_str().unwrap()).green()
        );
    };

    Ok(())
}

/// Undoes the last action made by Vento using the history file located on the Vento directory
pub fn undo() -> Result<()> {
    let lastpath: PathBuf = [
        common::env_config()?.vento_dir,
        Path::new("last").to_path_buf(),
    ]
    .iter()
    .collect();

    let lastfile = fs::read_to_string(lastpath)?;

    let mut contents = vec![];

    for line in lastfile.lines() {
        contents.push(line);
    }

    if contents.len() != 4 {
        bail!("Invalid history length".red());
    }

    match contents[3] {
        "take" => {
            let destpath = Path::new(contents[0]).to_path_buf();
            drop(&String::from(contents[1]), contents[2], destpath, false)?;
        }
        "drop" => {
            let path = vec![contents[0], contents[1]].join("/");
            take(&path, contents[2], false)?;
        }
        _ => bail!("Illegal action".red()),
    }

    println!("✅ {}", "Last action undone".green());

    Ok(())
}
