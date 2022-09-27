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

pub fn init() -> Result<()> {
    // Initializes Vento
    let ventodir = &common::env_config()?[0];

    if ventodir.is_dir() {
        // Checks if Vento has already been initialized and prompts the user if they want to initialize it again
        let mut answer = String::new();
        print!("⚠️  {} {}", "WARNING:".bold().red(), "Vento has already been initialized. Reinitializing will delete all files on the directory for Vento. Do you wish to proceed? (y/N) ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut answer)
            .context("Failed to read input")?;
        match answer.as_str().trim() {
            "y" | "Y" => fs::remove_dir_all(&ventodir)?,
            _ => process::exit(0),
        };
    };

    create_slots()?;
    Ok(())
}

pub fn list(slot: &str, dir: &str) -> Result<()> {
    // Lists files in inventory
    let mut slotdir: PathBuf = match slot {
        "active" | "a" => common::env_config()?[1].clone(),
        "inactive" | "i" => common::env_config()?[2].clone(),
        _ => PathBuf::new(),
    };

    let ventodir = &common::env_config()?[0];

    if !ventodir.is_dir() {
        bail!(
            "{}",
            "Vento not initialized. Run \"vento -i\" to initialize Vento.".red()
        );
    }

    if dir != "" {
        slotdir = [&slotdir, &Path::new(dir).to_path_buf()].iter().collect();
    }

    if dir.to_string().contains("..") {
        bail!("{}", "Cannot access parent.".red());
    }

    if slotdir.is_dir() {
        if fs::read_dir(&slotdir).unwrap().count() == 0 {
            println!(
                "🗃️  {}",
                format!(
                    "No files in {}{}.",
                    match slot {
                        "active" => format!("{}", slot).bold(),
                        "inactive" | _ => format!("{}", slot).blue().bold(),
                    },
                    if dir != "" {
                        if cfg!(windows) {
                            format!("\\{}", dir.to_string())
                        } else {
                            format!("/{}", dir.to_string())
                        }
                    } else {
                        "".to_string()
                    }
                )
                .green()
            );
        } else {
            // Checks if inventory selected exists
            println!(
                "🗃️  {}",
                format!(
                    "Files in {}{} ({}):",
                    match slot {
                        "active" => format!("{}", slot).bold(),
                        "inactive" | _ => format!("{}", slot).blue().bold(),
                    },
                    if dir != "" {
                        if cfg!(windows) {
                            format!("\\{}", dir.to_string())
                        } else {
                            format!("/{}", dir.to_string())
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
                        format!("")
                    }
                );
            }
        }
    } else {
        bail!(
            "{}",
            format!(
                "No such slot or directory. Valid slots are {} and {}.",
                "active".green().bold(),
                "inactive".blue().bold()
            )
            .red()
        );
    }
    Ok(())
}

pub fn switch() -> Result<()> {
    // Switches between inventory slots
    let ventodir = &common::env_config()?[0];
    let active = &common::env_config()?[1];
    let inactive = &common::env_config()?[2];
    let temp: PathBuf = [ventodir.to_path_buf(), Path::new("temp").to_path_buf()]
        .iter()
        .collect();

    let rename_error = "Vento was unable to switch slots. Try running vento init and try again";

    fs::rename(&active, &temp).context(rename_error)?;
    fs::rename(&inactive, &active).context(rename_error)?;
    fs::rename(&temp, &inactive).context(rename_error)?;

    println!("🎉 {}", "Switched inventory slots!".green());
    Ok(())
}

fn create_slots() -> Result<()> {
    // Used only on init. Creates all required directories.
    let active = &common::env_config()?[1];
    let inactive = &common::env_config()?[2];

    let initialize_error = "Vento was unable to initalize. Do you have the correct permissions?";

    fs::create_dir_all(active).context(initialize_error)?;
    fs::create_dir_all(inactive).context(initialize_error)?;

    println!("🎉 {}", "Vento has been succesfully initialized!".green());
    Ok(())
}
