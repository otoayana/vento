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

use std::{fs, process};
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use colored::Colorize;


pub fn init() { // Initializes Vento
    let ventodir: PathBuf = env_config();
    
    if ventodir.is_dir() { // Checks if Vento has already been initialized and prompts the user if they want to initialize it again
        let mut answer = String::new();
        print!("⚠️  {} {}", format!("WARNING:").bold().red(), "Vento has already been initialized. Reinitializing will delete all files on the directory for Vento. Do you wish to proceed? (y/N) ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut answer)
            .expect("❌ Failed to read input");
        match answer.as_str().trim() {
            "y" | "Y" => {fs::remove_dir_all(&ventodir).expect("❌ Vento was unable to initalize. Do you have the correct permissions?");},
            "n" | "N" | _ => process::exit(0)
        };
    };
    
    create_slots(ventodir);
}

pub fn list(slot: &str) { // Lists files in inventory
    let ventodir: PathBuf = env_config();
    let slotdir: PathBuf = [ventodir.to_path_buf(), Path::new(slot).to_path_buf()].iter().collect();

    if slotdir.is_dir() { // Checks if inventory selected exists
        println!("🗃️  {}", format!("Files in {} inventory:", match slot {
            "active" => format!("{}", slot).bold(),
            "inactive" | _ => format!("{}", slot).blue().bold()
        }).green());
        for file in fs::read_dir(&slotdir).unwrap() {
            println!("  - {}", file.unwrap().path().file_name().unwrap().to_os_string().into_string().unwrap());
        };
    } else {
        println!("❌ {}", format!("Vento was unable to read that slot. Valid slots are {} and {}.", format!("active").green(), format!("inactive").blue()).red());
    }
}

pub fn switch() { // Switches between inventory slots
    let ventodir: PathBuf = env_config();
    let active: PathBuf = [ventodir.to_path_buf(), Path::new("active").to_path_buf()].iter().collect();
    let temp: PathBuf = [ventodir.to_path_buf(), Path::new("temp").to_path_buf()].iter().collect();
    let inactive: PathBuf = [ventodir.to_path_buf(), Path::new("inactive").to_path_buf()].iter().collect();
    
    fs::rename(&active, &temp).expect("❌ Vento was unable to switch slots. Try running vento init and try again");
    fs::rename(&inactive, &active).expect("❌ Vento was unable to switch slots. Try running vento init and try again");
    fs::rename(&temp, &inactive).expect("❌ Vento was unable to switch slots. Try running vento init and try again");

    println!("🎉 {}", format!("Switched inventory slots!").green());
}

fn env_config() -> PathBuf { // Configures the directory for Vento
    let emptypath = PathBuf::new();
    let home = match dirs::home_dir() {
        Option::Some(dir) => dir,
        _ => PathBuf::new()
    };
    if home == emptypath {
        println!("❌ {}", format!("Vento was unable to detect your home folder. Have you configured your environment correctly?").red());
        process::exit(0);
    } else {
        return [home, Path::new(".vento").to_path_buf()].iter().collect();
    };
}

fn create_slots(dir: PathBuf) { // Used only on init. Creates all required directories.
    let active: PathBuf = [dir.to_path_buf(), Path::new("active").to_path_buf()].iter().collect();
    let inactive: PathBuf = [dir.to_path_buf(), Path::new("inactive").to_path_buf()].iter().collect();

    fs::create_dir_all(active).expect("❌ Vento was unable to initalize. Do you have the correct permissions?");
    fs::create_dir_all(inactive).expect("❌ Vento was unable to initalize. Do you have the correct permissions?");

    println!("🎉 {}", format!("Vento has been succesfully initialized!").green());
}
