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

use crate::message::{throw_error, ErrorType};
use anyhow::Result;
use colored::control::set_override;
use config::Config;
use rusqlite::Connection;
use serde::Deserialize;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Settings {
    pub vento_dir: PathBuf,
    pub active_dir: PathBuf,
    pub inactive_dir: PathBuf,
}

#[derive(Debug)]
pub struct HistoryData {
    pub id: i32,
    pub path: Option<PathBuf>,
    pub file: Option<String>,
    pub slot: Option<String>,
    pub action: Action,
    pub time: i64,
    pub current: i32,
}

pub struct DeserializedConfig {
    pub directory: String,
    pub display_dir: bool,
    pub history_display_dir: bool,
    pub display_emoji: bool,
    pub display_colors: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Item {
    display_dir: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct History {
    display_dir: bool,
}

#[derive(Debug)]
pub enum Action {
    Take,
    Drop,
    Switch,
}

/// Provides required variables for Vento
pub fn env_config() -> Result<Settings> {
    let home = match dirs::home_dir() {
        Option::Some(dir) => dir,
        _ => PathBuf::new(),
    };
    if home == PathBuf::new() {
        throw_error(ErrorType::NoHomeDirectory)?;
    };
    let custom_dir = Path::new(&parse_config()?.directory).to_path_buf();
    let vento_dir: PathBuf = if custom_dir != PathBuf::new() {
        Path::new(&custom_dir).to_path_buf()
    } else {
        [home, Path::new(".vento").to_path_buf()].iter().collect()
    };

    let active_dir = [&vento_dir, &Path::new("active").to_path_buf()]
        .iter()
        .collect();
    let inactive_dir = [&vento_dir, &Path::new("inactive").to_path_buf()]
        .iter()
        .collect();

    Ok(Settings {
        vento_dir,
        active_dir,
        inactive_dir,
    })
}

/// Handles reading the config file or variables for Vento.
pub fn parse_config() -> Result<DeserializedConfig> {
    let mut directory = String::new();
    let mut display_dir = true;
    let mut history_display_dir = true;
    let mut display_emoji = true;
    let mut display_colors = true;
    let mut config = match dirs::config_dir() {
        Option::Some(dir) => dir,
        _ => PathBuf::new(),
    };

    if config != PathBuf::new() {
        config.push("vento.toml");
        if config.is_file() {
            let settings = Config::builder()
                .add_source(config::File::with_name(
                    &config.as_path().display().to_string(),
                ))
                .add_source(config::Environment::with_prefix("VENTO"))
                .build()?;

            directory = match settings.get_string("directory") {
                Ok(value) => value,
                Err(_) => String::new(),
            };

            display_dir = settings.get_bool("item.display_dir").unwrap_or(true);
            history_display_dir = settings.get_bool("history.display_dir").unwrap_or(true);
            display_emoji = settings.get_bool("display_emoji").unwrap_or(true);
            display_colors = settings.get_bool("display_colors").unwrap_or(true);
        }
    };

    Ok(DeserializedConfig {
        directory,
        display_dir,
        history_display_dir,
        display_emoji,
        display_colors,
    })
}

/// Writes an action into the history database
pub fn history(data: HistoryData) -> Result<()> {
    let mut path = env_config()?.vento_dir;
    path.push("history.db3");
    let db = Connection::open(path)?;

    // Create table if it doesn't exist.
    db.execute(
        "CREATE TABLE IF NOT EXISTS history (
                id      INTEGER PRIMARY KEY,
                path    TEXT,
                file    TEXT,
                slot    TEXT,
                action  TEXT NOT NULL,
		time	INTEGER NOT NULL,
                current INTEGER NOT NULL)",
        (),
    )?;

    // Remove future actions
    let mut current = db.prepare("SELECT id FROM history WHERE current = 1")?;
    let actions = current.query_map([], |row| Ok(row.get(0)?))?;
    let lastaction: i64 = actions.last().unwrap_or(Ok(0))?;
    db.execute("DELETE FROM history WHERE id > ?1", [lastaction])?;

    // Unset current actions
    db.execute("UPDATE history SET current = 0 WHERE current = 1", ())?;

    // Insert action into table
    db.execute(
        "INSERT INTO history (path, file, slot, action, time, current) VALUES (?1, ?2, ?3, ?4, ?5, 1)",
        (
            data.path.unwrap_or(PathBuf::new()).to_str(),
            data.file,
            data.slot,
	    match data.action {
                Action::Take => "take",
                Action::Drop => "drop",
                Action::Switch => "switch",
            },
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::new(0, 0)).as_secs(),
        ),
    )?;

    Ok(())
}

/// Gets current directory for commands
pub fn get_current_dir() -> Result<PathBuf> {
    let currentdir = match current_dir() {
        Ok(dir) => dir,
        Err(_) => PathBuf::new(),
    };

    if currentdir == PathBuf::new() {
        throw_error(ErrorType::NoCurrentDirectory)?;
    }

    Ok(currentdir)
}

/// Sets color override if display_colors is disabled
pub fn override_color() -> Result<()> {
    if !parse_config()?.display_colors {
        set_override(false)
    }

    Ok(())
}
