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
use config::Config;
use std::path::{Path, PathBuf};

pub fn env_config() -> Result<Vec<PathBuf>> {
    // Configures the directories for Vento
    let home = match dirs::home_dir() {
        Option::Some(dir) => dir,
        _ => PathBuf::new(),
    };
    if home == PathBuf::new() {
        bail!("{}", "Vento was unable to detect your home folder. Have you configured your environment correctly?".red());
    };
    let vento_dir: PathBuf;
    let custom_dir = Path::new(&dir_config()?).to_path_buf();
    if custom_dir != PathBuf::new() {
        vento_dir = Path::new(&custom_dir).to_path_buf();
    } else {
        vento_dir = [home, Path::new(".vento").to_path_buf()].iter().collect();
    }
    let active_dir = [&vento_dir, &Path::new("active").to_path_buf()]
        .iter()
        .collect();
    let inactive_dir = [&vento_dir, &Path::new("inactive").to_path_buf()]
        .iter()
        .collect();

    Ok(vec![vento_dir, active_dir, inactive_dir])
}

fn dir_config() -> Result<String> {
    let mut result = String::new();
    let mut config = match dirs::config_dir() {
        Option::Some(dir) => dir,
        _ => PathBuf::new(),
    };

    if &config != &PathBuf::new() {
        config.push("vento.toml");
        if config.is_file() {
            let settings = Config::builder()
                .add_source(config::File::with_name(
                    &config.as_path().display().to_string(),
                ))
                .add_source(config::Environment::with_prefix("VENTO"))
                .build()?;

            result = match settings.get_string("directory") {
                Ok(value) => value,
                Err(_) => String::new(),
            };
        }
    };

    Ok(result)
}
