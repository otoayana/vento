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

use colored::Colorize;
use std::env;
use std::path::Path;
use std::process;

mod common;
mod inv;
mod item;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        // If the vector for the arguments the command is taking is larger than 2, it most likely means the user has provided an argument
        match args[1].as_str() {
            "help" | "h" => help(),
            "init" | "i" => inv::init(),
            "list" | "l" => {
                if args.len() == 3 {
                    // If the user has provided a slot, it'll use it. Otherwise, it'll default to the active slot
                    inv::list(args[2].as_str());
                } else {
                    inv::list("active");
                };
            }
            "switch" | "s" => inv::switch(),
            "take" | "t" => {
                if args.len() == 3 {
                    // Similar thing with list, but change it with a file and it will show an error instead of defaulting to anything
                    item::take(&args[2]);
                } else {
                    println!("❌ {}", format!("You need to specify a file.").red())
                };
            }
            "drop" | "d" => {
                if args.len() == 3 {
                    // Tries to get the current directory if the user hasn't provided a "landing location"
                    item::drop(
                        &args[2],
                        match env::current_dir() {
                            Ok(dir) => dir,
                            Err(_) => {
                                println!("❌ {}", format!("Vento was unable to detect your current directory. Have you configured your environment correctly?").red());
                                process::exit(1);
                            }
                        },
                    );
                } else if args.len() == 4 {
                    item::drop(&args[2], Path::new(&args[3]).to_path_buf());
                } else {
                    println!("❌ {}", format!("You need to specify a file.").red())
                };
            }
            _ => {
                println!("❔ Command not found. Type \"vento help\" to see all commands available.")
            }
        }
    } else {
        // If the user provides no commands, it'll fall back to the help guide
        help();
    }
}

fn help() {
    // A quick guide to move around in Vento
    println!(
        "{}, a CLI inventory for your files
© 2022 Lux Aliaga. Licensed under GPLv3

{}
    - {}: Takes a file or directory and saves it in your inventory
    - {}: Drops a file off of your inventory
    - {}: Lists files in selected inventory
    - {}: Switches slots
    - {}: Initializes Vento
    - {}: Displays this message",
        format!("Vento").bold().blue(),
        format!("Usage:").bold(),
        format!("take | t <file | directory>").bold().green(),
        format!("drop | d <file | directory> [destination]")
            .bold()
            .green(),
        format!("list | l [slot]").bold().green(),
        format!("switch | s").bold().green(),
        format!("init | i").bold().green(),
        format!("help | h").bold().green()
    );
}
