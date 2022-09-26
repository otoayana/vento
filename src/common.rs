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
use std::path::{Path, PathBuf};

pub fn env_config() -> Result<Vec<PathBuf>> {
    // Configures the directories for Vento
    let emptypath = PathBuf::new();
    let home = match dirs::home_dir() {
        Option::Some(dir) => dir,
        _ => PathBuf::new(),
    };
    if home == emptypath {
        bail!("❌ {}", "Vento was unable to detect your home folder. Have you configured your environment correctly?".red());
    };
    let vento_dir = [home, Path::new(".vento").to_path_buf()].iter().collect();
    let active_dir = [&vento_dir, &Path::new("active").to_path_buf()]
        .iter()
        .collect();
    let inactive_dir = [&vento_dir, &Path::new("inactive").to_path_buf()]
        .iter()
        .collect();

    Ok(vec![vento_dir, active_dir, inactive_dir])
}
