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
use colored::Colorize;

mod inv;

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
    - {}: Initializes Vento
    - {}: Lists files in selected inventory
    - {}: Switches slots
    - {}: Displays this message",
       format!("Vento").bold().blue(),
       format!("Usage:").bold(),
       format!("init").bold().green(),
       format!("list [slot]").bold().green(),
       format!("switch").bold().green(),
       format!("help").bold().green());
}
