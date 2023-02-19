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

use anyhow::Result;
use std::{env, path::PathBuf};
use vento::{
    archive,
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
                "-e" | "--export-inv" => match args.len() {
                    4 => archive::export_inv(&args[2], PathBuf::from(&args[3]), true)?,
                    3 => match args[2].as_str() {
                        "active" | "a" | "inactive" | "i" => {
                            let mut path = PathBuf::from(match args[2].as_str() {
                                "a" => "active",
                                "i" => "inactive",
                                _ => &args[2],
                            });
                            path.set_extension("tar.xz");
                            archive::export_inv(&args[2], path, true)?
                        }
                        _ => archive::export_inv("active", PathBuf::from(&args[2]), true)?,
                    },
                    2 => archive::export_inv("active", PathBuf::from("active.tar.xz"), true)?,
                    _ => throw_error(ErrorType::TooManyArgs)?,
                },
                "-E" | "--export-install" => match args.len() {
                    3 => archive::export_install(PathBuf::from(&args[2]), true)?,
                    2 => archive::export_install(PathBuf::from("vento.tar.xz"), true)?,
                    _ => throw_error(ErrorType::TooManyArgs)?,
                },
                "-g" | "--import-inv" => match args.len() {
                    4 => archive::import_inv(PathBuf::from(&args[2]), &args[3], true)?,
                    3 => archive::import_inv(PathBuf::from(&args[2]), "active", true)?,
                    2 => throw_error(ErrorType::SpecifyFile)?,
                    _ => throw_error(ErrorType::TooManyArgs)?,
                },
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
