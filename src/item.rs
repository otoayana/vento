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
use anyhow::{bail, Context, Result};
use colored::Colorize;
use fs_extra::dir::{move_dir, CopyOptions};
use std::fs;
use std::path::{Path, PathBuf};

pub fn take(file: &String, slot: &String) -> Result<()> {
    // Takes a file or directory
    let ventodir = &common::env_config()?[0];

    if !ventodir.is_dir() {
        // Detects if Vento hasn't been initialized and bails if so
        bail!(
            "{}",
            "Vento not initialized. Run \"vento -i\" to initialize Vento.".red()
        );
    };
    let slotdir: PathBuf = match slot.as_str() {
        "active" | "a" => common::env_config()?[1].clone(),
        "inactive" | "i" => common::env_config()?[2].clone(),
        _ => PathBuf::new(),
    };

    if !slotdir.is_dir() {
        // Detects if the slot provided exists
        bail!(
            "{}",
            format!(
                "No such slot. Valid slots are {} and {}.",
                "active".green().bold(),
                "inactive".blue().bold()
            )
            .red()
        );
    };

    let sourcepath: PathBuf = Path::new(&file).to_path_buf();
    let destpath: PathBuf = [
        &slotdir,
        &Path::new(
            &Path::new(&file)
                .file_name()
                .unwrap()
                .to_os_string()
                .to_str()
                .unwrap(),
        )
        .to_path_buf(),
    ]
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
        fs::copy(&file, &destpath).context("Vento was unable to copy the file.")?;
        fs::remove_file(&file).context("Vento was unable to remove the file.")?;
    } else if sourcepath.is_dir() {
        let options = CopyOptions::new();
        move_dir(&file, &slotdir, &options).context("Vento was unable to move the directory.")?;
    } else {
        bail!("{}", "No such file or directory.".red());
    }

    Ok(())
}

pub fn drop(file: &String, slot: &String, dest: PathBuf) -> Result<()> {
    // Drops a file or directory
    let ventodir = &common::env_config()?[0];

    if !ventodir.is_dir() {
        // Detects if Vento hasn't been initialized and bails if so
        bail!(
            "{}",
            "Vento not initialized. Run \"vento -i\" to initialize Vento.".red()
        );
    };

    let slotdir: PathBuf = match slot.as_str() {
        "active" | "a" => common::env_config()?[1].clone(),
        "inactive" | "i" => common::env_config()?[2].clone(),
        _ => PathBuf::new(),
    };

    if !slotdir.is_dir() {
        // Detects if the slot provided exists
        bail!(
            "{}",
            format!(
                "No such slot. Valid slots are {} and {}.",
                "active".green().bold(),
                "inactive".blue().bold()
            )
            .red()
        );
    };

    let sourcepath: PathBuf = [&slotdir, &Path::new(file).to_path_buf()].iter().collect();
    let destpath: PathBuf = [
        Path::new(&dest).to_path_buf(),
        Path::new(file).to_path_buf(),
    ]
    .iter()
    .collect();

    if Path::exists(&destpath) {
        // Checks if there's a file with the same name in the destination path.
        bail!("{}", "A file with the same name already exists in the destination! Try renaming it or dropping this file somewhere else.".red());
    }

    if sourcepath.is_file() | sourcepath.is_symlink() {
        // Checks the path's file type
        fs::copy(&sourcepath, &destpath).context("Vento was unable to copy the file.")?;
        fs::remove_file(&sourcepath).context("Vento was unable to remove the file.")?;
    } else if sourcepath.is_dir() {
        let destpath: PathBuf = Path::new(&dest).to_path_buf();
        let options = CopyOptions::new();
        move_dir(&sourcepath, &destpath, &options)
            .context("Vento was unable to move the directory.")?;
    } else {
        bail!("{}", "No such file or directory.".red());
    }
    Ok(())
}
