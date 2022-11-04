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
use size_format::SizeFormatterBinary;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::{fs, process};

/// Initializes Vento by creating the respective directories it will use
pub fn init() -> Result<()> {
    let ventodir = &common::env_config()?.vento_dir;

    if ventodir.is_dir() {
        // Checks if Vento has already been initialized and prompts the user if they want to initialize it again
        let mut answer = String::new();
        print!("⚠️  {} Vento has already been initialized. Reinitializing will delete all files on the directory for Vento. Do you wish to proceed? (y/N) ", "WARNING:".bold().red());
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut answer)?;
        match answer.as_str().trim() {
            "y" | "Y" => fs::remove_dir_all(&ventodir)?,
            _ => process::exit(0),
        };
    };

    create_slots()?;
    Ok(())
}

/// Lists files in the provided slot and/or directory
pub fn list(slot: &str, dir: &str) -> Result<()> {
    let ventodir = &common::env_config()?.vento_dir;

    if !ventodir.is_dir() {
        // Detects if Vento hasn't been initialized and bails if so
        bail!(
            "{}",
            "Vento not initialized. Run \"vento -i\" to initialize Vento".red()
        );
    }

    let mut slotdir: PathBuf = match slot {
        "active" | "a" => common::env_config()?.active_dir,
        "inactive" | "i" => common::env_config()?.inactive_dir,
        _ => PathBuf::new(),
    };

    if !dir.is_empty() {
        // Detects if the directory argument is not empty, and if so appends the path provided to the slot directory variable
        slotdir = [&slotdir, &Path::new(dir).to_path_buf()].iter().collect();
    }

    if dir.to_string().contains("..") {
        // Basically preventing from listing anything out of bounds. ls and dir exist for that
        bail!("{}", "Cannot access parent".red());
    }

    if !slotdir.is_dir() {
        // Detects if the consulted slot or directory exists
        bail!(
            "{}",
            format!(
                "No such slot or directory. Valid slots are {} and {}",
                "active".green().bold(),
                "inactive".blue().bold()
            )
            .red()
        );
    };

    if fs::read_dir(&slotdir).unwrap().count() == 0 {
        // Detects if the slot or directory has any contents
        println!(
            "🗃️  {}",
            format!(
                "No files in {}{}",
                match slot {
                    "active" => slot.bold(),
                    _ => slot.blue().bold(),
                },
                if !dir.is_empty() {
                    if cfg!(windows) {
                        format!("\\{}", dir)
                    } else {
                        format!("/{}", dir)
                    }
                } else {
                    "".to_string()
                }
            )
            .green()
        );
    } else {
        println!(
            "🗃️  {}",
            format!(
                "Files in {}{} ({}):",
                match slot {
                    "active" => slot.bold(),
                    _ => slot.blue().bold(),
                },
                if !dir.is_empty() {
                    if cfg!(windows) {
                        format!("\\{}", dir)
                    } else {
                        format!("/{}", dir)
                    }
                } else {
                    " inventory".to_string()
                },
                format!("{}", fs::read_dir(&slotdir).unwrap().count())
                    .white()
                    .bold()
            )
            .green()
        );
        for file in fs::read_dir(&slotdir).unwrap() {
            let file = file.unwrap().path();

            println!(
                "   - [{}] {}{}",
                if file.clone().is_dir() {
                    "D".blue()
                } else if file.clone().is_symlink() {
                    "S".yellow()
                } else {
                    "F".green()
                },
                file.clone()
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap(),
                if file.clone().is_file() {
                    format!(
                        " ({}B)",
                        SizeFormatterBinary::new(file.clone().metadata().unwrap().len())
                    )
                } else {
                    String::new()
                }
            );
        }
    }
    Ok(())
}

/// Switches inevntory slots between each other, making the currently active inventory inactive and viceversa
pub fn switch(message: bool) -> Result<()> {
    let ventodir = &common::env_config()?.vento_dir;
    let active = &common::env_config()?.active_dir;
    let inactive = &common::env_config()?.inactive_dir;
    let temp: PathBuf = [ventodir.to_path_buf(), Path::new("temp").to_path_buf()]
        .iter()
        .collect();

    let rename_error = "Vento was unable to switch slots. Try running \"vento -i\" and try again";

    fs::rename(&active, &temp).context(rename_error)?;
    fs::rename(&inactive, &active).context(rename_error)?;
    fs::rename(&temp, &inactive).context(rename_error)?;

    common::history(common::HistoryData {
        path: PathBuf::new(),
        file: String::new(),
        slot: String::new(),
        action: common::Action::Switch,
    })?;

    if message {
        println!("✅ {}", "Switched inventory slots!".green());
    }
    Ok(())
}

// Used only on init. Creates all required directories
fn create_slots() -> Result<()> {
    let active = &common::env_config()?.active_dir;
    let inactive = &common::env_config()?.inactive_dir;

    fs::create_dir_all(active)?;
    fs::create_dir_all(inactive)?;

    println!("🎉 {}", "Vento has been succesfully initialized!".green());
    Ok(())
}
