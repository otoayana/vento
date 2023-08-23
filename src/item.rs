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

use super::{
    common::{env_config, history, parse_config, Action, HistoryData},
    message::{append_emoji, throw_error, EmojiType, ErrorType},
};
use anyhow::{bail, Result};
use colored::Colorize;
use fs_extra::dir::{move_dir, CopyOptions};
use std::fs;
use std::path::{Path, PathBuf};

/// Takes a file or directory and stores it in an inventory slot
pub fn take(file: &String, slot: &str, message: bool) -> Result<()> {
    let ventodir = &env_config()?.vento_dir;

    if !ventodir.is_dir() {
        // Detects if Vento hasn't been initialized and bails if so
        throw_error(ErrorType::NotInitialized)?;
    };
    let slotdir: PathBuf = match slot {
        "active" | "a" => env_config()?.active_dir,
        "inactive" | "i" => env_config()?.inactive_dir,
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
        throw_error(ErrorType::ExistsInventory)?;
    }

    if sourcepath.is_file() | sourcepath.is_symlink() {
        // Checks the path's file type
        fs::copy(file, &destpath)?;
        fs::remove_file(file)?;
    } else if sourcepath.is_dir() {
        let options = CopyOptions::new();
        move_dir(file, &slotdir, &options)?;
    } else {
        throw_error(ErrorType::NoFileOrDir)?;
    }

    history(HistoryData {
        path: sourcelocation.clone(),
        file: String::from(filename),
        slot: String::from(slot),
        action: Action::Take,
    })?;

    if message {
        println!(
            "{}{} {} {}{} {} {}",
            append_emoji(EmojiType::Success)?,
            "Took".green(),
            &filename.bold(),
            match parse_config()?.display_dir {
                true => format! {"{} {} ",
                    "from".green(),
                    &sourcelocation.to_str().unwrap(),
                },
                _ => String::new(),
            },
            "to".green(),
            match slot {
                "active" => slot.green(),
                "inactive" => slot.blue(),
                _ => slot.red(),
            }
            .bold(),
            "slot".green()
        );
    }

    Ok(())
}

/// Drops a file or directory and stores it in an inventory slot
pub fn drop(file: &String, slot: &str, dest: PathBuf, message: bool) -> Result<()> {
    // Drops a file or directory
    let ventodir = &env_config()?.vento_dir;

    if !ventodir.is_dir() {
        // Detects if Vento hasn't been initialized and bails if so
        throw_error(ErrorType::NotInitialized)?;
    };

    let slotdir: PathBuf = match slot {
        "active" | "a" => env_config()?.active_dir,
        "inactive" | "i" => env_config()?.inactive_dir,
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
        throw_error(ErrorType::ExistsDestination)?;
    }

    if sourcepath.is_file() | sourcepath.is_symlink() {
        // Checks the path's file type
        fs::copy(&sourcepath, &destpath)?;
        fs::remove_file(&sourcepath)?;
    } else if sourcepath.is_dir() {
        let destpath: PathBuf = Path::new(&dest).to_path_buf();
        let options = CopyOptions::new();
        move_dir(&sourcepath, destpath, &options)?;
    } else {
        throw_error(ErrorType::NoFileOrDir)?;
    }

    destpath.pop();

    history(HistoryData {
        path: destpath.clone(),
        file: String::from(file),
        slot: String::from(slot),
        action: Action::Drop,
    })?;

    if message {
        println!(
            "{}{} {} {} {} {}{}",
            append_emoji(EmojiType::Success)?,
            "Dropped".green(),
            &file.bold(),
            "from".green(),
            match slot {
                "active" => slot.green(),
                "inactive" => slot.blue(),
                _ => slot.red(),
            }
            .bold(),
            "slot".green(),
            match parse_config()?.display_dir {
                true => format! {"{} {} ",
                    " into".green(),
                    &destpath.to_str().unwrap(),
                },
                _ => String::new(),
            },
        );
    };

    Ok(())
}
