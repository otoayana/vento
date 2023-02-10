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
    help, item,
};

fn main() -> Result<()> {
    // Handles args in Vento
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        if args[1].contains("--slot=") {
            // Checks if the user has provided the long argument "--slot="
            match args.len() {
                3 => item::take(&args[2], &args[1].replace("--slot=", ""), true)?,
                2 => throw_error(ErrorType::SpecifyFile)?,
                _ => throw_error(ErrorType::TooManyArgs)?,
            };
        } else {
            match args[1].as_str() {
                "--help" | "-h" => help::take()?,
                "-s" => match args.len() {
                    4 => item::take(&args[3], &args[2], true)?,
                    3 => throw_error(ErrorType::SpecifyFile)?,
                    2 => throw_error(ErrorType::SpecifySlot)?,
                    _ => throw_error(ErrorType::TooManyArgs)?,
                },
                _ => match args.len() {
                    2 => item::take(&args[1], &String::from("active"), true)?,
                    _ => throw_error(ErrorType::TooManyArgs)?,
                },
            }
        }
    } else {
        // If the user provides no arguments, Take will display the help message.
        help::take()?;
    }
    Ok(())
}
