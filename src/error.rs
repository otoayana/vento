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
}

pub fn throw_error(error: ErrorType) -> Result<()> {
    bail!(
        "{}",
        match error {
            ErrorType::TooManyArgs => "Too many arguments",
            ErrorType::SpecifySlot => "You need to specify a file",
            ErrorType::SpecifyFile => "You need to specify a slot",
            ErrorType::NoCurrentDirectory => "Vento was unable to detect your current directory. Have you configured your environment correctly?",
        }
        .red()
    );
}
