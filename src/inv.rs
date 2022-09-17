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
use colored::Colorize;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::{fs, process};

pub fn init() {
    // Initializes Vento
    let ventodir = &common::env_config()[0];

    if ventodir.is_dir() {
        // Checks if Vento has already been initialized and prompts the user if they want to initialize it again
        let mut answer = String::new();
        print!("⚠️  {} {}", format!("WARNING:").bold().red(), "Vento has already been initialized. Reinitializing will delete all files on the directory for Vento. Do you wish to proceed? (y/N) ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut answer)
            .expect("❌ Failed to read input");
        match answer.as_str().trim() {
            "y" | "Y" => {
                fs::remove_dir_all(&ventodir).expect(
                    "❌ Vento was unable to initalize. Do you have the correct permissions?",
                );
            }
            "n" | "N" | _ => process::exit(0),
        };
    };

    create_slots();
}

pub fn list(slot: &str) {
    // Lists files in inventory
    let slotdir: PathBuf = match slot {
        "active" | "a" => common::env_config()[1].clone(),
        "inactive" | "i" => common::env_config()[2].clone(),
        _ => PathBuf::new(),
    };

    if slotdir.is_dir() {
        // Checks if inventory selected exists
        println!(
            "🗃️  {}",
            format!(
                "Files in {} inventory:",
                match slot {
                    "active" => format!("{}", slot).bold(),
                    "inactive" | _ => format!("{}", slot).blue().bold(),
                }
            )
            .green()
        );
        for file in fs::read_dir(&slotdir).unwrap() {
            let file = file.unwrap().path();

            println!(
                "  - {} ({})",
                file.clone()
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap(),
                if file.clone().is_dir() {
                    format!("dir").blue()
                } else if file.clone().is_symlink() {
                    format!("symlink").yellow()
                } else {
                    format!("file").green()
                }
            );
        }
    } else {
        println!(
            "❌ {}",
            format!(
                "Vento was unable to read that slot. Valid slots are {} and {}.",
                format!("active").green(),
                format!("inactive").blue()
            )
            .red()
        );
    }
}

pub fn switch() {
    // Switches between inventory slots
    let ventodir = &common::env_config()[0];
    let active = &common::env_config()[1];
    let inactive = &common::env_config()[2];
    let temp: PathBuf = [ventodir.to_path_buf(), Path::new("temp").to_path_buf()]
        .iter()
        .collect();

    fs::rename(&active, &temp)
        .expect("❌ Vento was unable to switch slots. Try running vento init and try again");
    fs::rename(&inactive, &active)
        .expect("❌ Vento was unable to switch slots. Try running vento init and try again");
    fs::rename(&temp, &inactive)
        .expect("❌ Vento was unable to switch slots. Try running vento init and try again");

    println!("🎉 {}", format!("Switched inventory slots!").green());
}

fn create_slots() {
    // Used only on init. Creates all required directories.
    let active = &common::env_config()[1];
    let inactive = &common::env_config()[2];

    fs::create_dir_all(active)
        .expect("❌ Vento was unable to initalize. Do you have the correct permissions?");
    fs::create_dir_all(inactive)
        .expect("❌ Vento was unable to initalize. Do you have the correct permissions?");

    println!(
        "🎉 {}", 
        format!("Vento has been succesfully initialized!").green()
    );
}
