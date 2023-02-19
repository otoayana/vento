/*
 * Vento, a CLI inventory for your files.
 * Copyright (C) 2023 Lux Aliaga
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

pub enum ErrorType {
    TooManyArgs,
    SpecifySlot,
    SpecifyFile,
    NoCurrentDirectory,
    NoHomeDirectory,
    InvalidHistoryLength,
    IllegalAction,
    NotInitialized,
    NoAccessParent,
    ExistsInventory,
    ExistsDestination,
    NoFileOrDir,
}

/// Displays an error and exits
pub fn throw_error(error: ErrorType) -> Result<()> {
    bail!(
        "{}",
        match error {
            ErrorType::TooManyArgs => "Too many arguments",
            ErrorType::SpecifyFile => "You need to specify a file",
            ErrorType::SpecifySlot => "You need to specify a slot",
            ErrorType::NoCurrentDirectory => "Vento was unable to detect your current directory. Have you configured your environment correctly?",
            ErrorType::NoHomeDirectory => "Vento was unable to detect your home directory. Have you configured your environment correctly?",
            ErrorType::InvalidHistoryLength => "Invalid history length",
            ErrorType::IllegalAction => "Illegal action",
            ErrorType::NotInitialized => "Vento not initialized. Run \"vento -i\" to initialize Vento",
            ErrorType::NoAccessParent => "Cannot access parent",
            ErrorType::ExistsInventory => "A file with the same name already exists in your inventory!",
            ErrorType::ExistsDestination => "A file with the same name already exists in the destination! Try renaming it or dropping this file somewhere else",
            ErrorType::NoFileOrDir => "No such file or directory",
        }
        .red()
    );
}
