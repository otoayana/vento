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

use anyhow::{bail, Result};
use colored::Colorize;
use std::env;
use vento::{
    error::{throw_error, ErrorType},
    help, history, inv,
};

fn main() -> Result<()> {
    // Handles args in Vento
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        // If the vector for the arguments the command is taking is larger than 2, it most likely means the user has provided an argument
        if args[1].contains("--slot=") {
            // Checks if the user has provided the long argument "--slot="
            match args.len() {
                3 => inv::list(&args[1].replace("--slot=", ""), &args[2])?,
                2 => inv::list(&args[1].replace("--slot=", ""), "")?,
                _ => throw_error(ErrorType::TooManyArgs)?,
            };
        } else {
            match args[1].as_str() {
                "-h" | "--help" => help::vento()?,
                "-i" | "--init" => inv::init()?,
                "-c" | "--switch" => inv::switch(true)?,
                "-u" | "--undo" => history::undo()?,
                "-s" => match args.len() {
                    4 => inv::list(&args[2], &args[3])?,
                    3 => inv::list(&args[2], "")?,
                    2 => throw_error(ErrorType::SpecifySlot)?,
                    _ => throw_error(ErrorType::TooManyArgs)?,
                },
                _ => inv::list("active", args[1].as_str())?,
            }
        }
    } else {
        // If the user provides no arguments, Vento will display the files in the active slot.
        inv::list("active", "")?;
    }
    Ok(())
}
