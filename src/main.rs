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

use std::env;
use std::process;
use std::path::{Path, PathBuf};
use colored::Colorize;

mod inv;
mod item;
mod common;

fn main() {
   let args: Vec<String> = env::args().collect();
   if args.len() >= 2 {
       match args[1].as_str() {
            "help" => help(),
            "init" => inv::init(),
            "list" => {
                if args.len() == 3 {
                    inv::list(args[2].as_str());
                } else {
                    inv::list("active");
                };
            },
            "switch" => inv::switch(),
            "take" => {
                if args.len() == 3 {
                    item::take(&args[2]);
                } else {
                    println!("❌ {}", format!("You need to specify a file.").red())
                };
            },
            "drop" => {
                if args.len() == 3 {
                    item::drop(&args[2], match env::current_dir() {
                        Ok(dir) => dir,
                        Err(_) => {
                            println!("❌ {}", format!("Vento was unable to detect your current directory. Have you configured your environment correctly?").red());
                            process::exit(1);
                        }
                    });
                } else if args.len() == 4 {
                    item::drop(&args[2], Path::new(&args[3]).to_path_buf());
                } else {
                    println!("❌ {}", format!("You need to specify a file.").red())
                };
            },
            _ => println!("❔ Command not found. Type \"vento help\" to see all commands available.")
       }
   } else {
        help();
   }
}

fn help() {
    println!("{}, a CLI inventory for your files
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
       format!("take <file | directory>").bold().green(),
       format!("drop <file | directory> [destination]").bold().green(),
       format!("list [slot]").bold().green(),
       format!("switch").bold().green(),
       format!("init").bold().green(),
       format!("help").bold().green());
}
