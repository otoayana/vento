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

use crate::common;
use anyhow::Result;
use colored::Colorize;
use std::{fs::File, path::PathBuf};
use xz2::write::XzEncoder;

pub fn export_inv(slot: &str, output: PathBuf, message: bool) -> Result<()> {
    let slotdir: PathBuf = match slot {
        "active" | "a" => common::env_config()?.active_dir,
        "inactive" | "i" => common::env_config()?.inactive_dir,
        _ => PathBuf::new(),
    };

    let archive = File::create(&output)?;
    let enc = XzEncoder::new(archive, 9);
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("", slotdir)?;

    if message {
        println!(
            "✅ {} {} {} {}",
            "Exported".green(),
            match slot {
                "a" | "active" => "active".green(),
                "i" | "inactive" => "inactive".blue(),
                _ => slot.red(),
            }
            .bold(),
            "slot into".green(),
            &output.to_str().unwrap()
        );
    };
    Ok(())
}
