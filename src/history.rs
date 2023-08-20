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

use crate::{
    common, inv, item,
    message::{append_emoji, throw_error, EmojiType, ErrorType},
};
use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

/// Undoes the last action made by Vento using the history file located on the Vento directory
pub fn undo() -> Result<()> {
    let lastpath: PathBuf = [
        common::env_config()?.vento_dir,
        Path::new("last").to_path_buf(),
    ]
    .iter()
    .collect();

    let lastfile = fs::read_to_string(lastpath)?;

    let mut contents = vec![];

    for line in lastfile.lines() {
        contents.push(line);
    }

    if contents.len() != 4 {
        throw_error(ErrorType::InvalidHistoryLength)?;
    }

    match contents[3] {
        "take" => {
            let destpath = Path::new(contents[0]).to_path_buf();
            item::drop(&String::from(contents[1]), contents[2], destpath, false)?;
        }
        "drop" => {
            let path = vec![contents[0], contents[1]].join("/");
            item::take(&path, contents[2], false)?;
        }
        "switch" => {
            inv::switch(false)?;
        }
        _ => throw_error(ErrorType::IllegalAction)?,
    }

    println!(
        "{}{}{}{}",
        append_emoji(EmojiType::Success)?,
        match contents[3] {
            "take" => "Take",
            "drop" => "Drop",
            "switch" => "Switch",
            _ => "Unknown",
        }
        .bold(),
        " action undone".green(),
        match contents[3] {
            "take" => format!(
                "{}{}{}{}{}{}{}",
                " (".green(),
                contents[1].bold(),
                ", from ".green(),
                contents[0],
                " to ".green(),
                match contents[2] {
                    "active" => contents[2].green(),
                    "inactive" => contents[2].blue(),
                    _ => contents[2].red(),
                }
                .bold(),
                " slot)".green(),
            ),
            "drop" => format!(
                "{}{}{}{}{}{}{}",
                " (".green(),
                contents[1].bold(),
                ", from ".green(),
                match contents[2] {
                    "active" => contents[2].green(),
                    "inactive" => contents[2].blue(),
                    _ => contents[2].red(),
                }
                .bold(),
                " slot to ".green(),
                contents[0],
                ")".green(),
            ),
            _ => String::from(""),
        }
    );

    Ok(())
}
