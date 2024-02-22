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
    common::{self, env_config, parse_config, Action, HistoryData},
    inv, item,
    message::{append_emoji, throw_error, EmojiType, ErrorType},
};
use anyhow::Result;
use chrono::prelude::*;
use colored::Colorize;
use rusqlite::Connection;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Undoes actions made by Vento using the history database located on the Vento directory
pub fn undo(steps: usize) -> Result<()> {
    let path: PathBuf = [
        env_config()?.vento_dir,
        Path::new("history.db3").to_path_buf(),
    ]
    .iter()
    .collect();
    let db = Connection::open(path)?;

    // Determine if step amount is greater than the position of the action
    let mut current = db.prepare("SELECT id FROM history WHERE current = 1")?;
    let actions = current.query_map([], |row| row.get(0))?;
    let last_action: usize = actions.last().unwrap_or(Ok(0))?;

    if last_action <= steps {
        throw_error(ErrorType::InvalidStepsLength)?;
    }

    let final_dest = last_action - steps;

    // Calculates how many actions need to be undone
    let mut undo_queue_transaction = db.prepare(
        "SELECT id, path, file, slot, action FROM history WHERE id > ?2 AND id <= ?1 ORDER BY id DESC",
    )?;
    let undo_queue = undo_queue_transaction.query_map([last_action, final_dest], |row| {
        Ok(HistoryData {
            id: row.get(0)?,
            path: Some(PathBuf::from(row.get::<_, String>(1)?)),
            file: row.get(2)?,
            slot: row.get(3)?,
            action: match row.get::<_, String>(4)?.as_str() {
                "take" => Action::Take,
                "drop" => Action::Drop,
                "switch" => Action::Switch,
                _ => unreachable!(),
            },
            time: 0,
            current: 0,
        })
    })?;

    // Undoes actions for each step
    for raw_step in undo_queue {
        let step = raw_step?;

        match step.action {
            Action::Take => {
                item::drop(
                    &step.file.unwrap(),
                    &step.slot.unwrap(),
                    step.path.unwrap(),
                    false,
                    false,
                    false,
                )?;
            }
            Action::Drop => {
                let path: String = [
                    String::from(step.path.unwrap().to_str().unwrap()),
                    step.file.unwrap(),
                ]
                .join("/");
                item::take(&path, step.slot.unwrap().as_str(), false, false, false)?;
            }
            Action::Switch => inv::switch(false, false)?,
        }

        db.execute("UPDATE history SET current = 0 WHERE current = 1", ())?;
        db.execute(
            "UPDATE history SET current = 1 WHERE id = ?1",
            [step.id - 1],
        )?;
    }

    // Prepares to display details of the final position
    let mut final_transaction = db.prepare("SELECT * FROM history WHERE current = 1")?;
    let final_action_iter = final_transaction.query_map([], |row| {
        Ok(HistoryData {
            id: row.get(0)?,
            path: Some(PathBuf::from(row.get::<_, String>(1)?)),
            file: row.get(2)?,
            slot: row.get(3)?,
            action: match row.get::<_, String>(4)?.as_str() {
                "take" => Action::Take,
                "drop" => Action::Drop,
                "switch" => Action::Switch,
                _ => unreachable!(),
            },
            time: row.get(5)?,
            current: row.get::<_, i32>(5)?,
        })
    })?;

    let final_action = final_action_iter.last().unwrap()?;

    // Formats the current action's timestamp to readable, local time
    let timestamp = final_action.time;
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0);
    let datetime = TimeZone::from_utc_datetime(&Local, &naive.unwrap());
    let newdate = datetime.format("%Y-%m-%d, %H:%M:%S");

    println!(
        "{}{}{}{}{}{}",
        append_emoji(EmojiType::Success)?,
        "Rolled back to ".green(),
        match final_action.action {
            Action::Take => "Take",
            Action::Drop => "Drop",
            Action::Switch => "Switch",
        }
        .bold(),
        " action, on ".green(),
        newdate,
        match final_action.action {
            Action::Take => format!(
                "{}{}{}{}{}{}{}",
                " (".green(),
                final_action.file.unwrap().bold(),
                ", ".green(),
                match parse_config()?.history_display_dir {
                    true => format!(
                        "{} {} ",
                        "from".green(),
                        final_action.path.unwrap().to_str().unwrap(),
                    ),
                    _ => String::new(),
                },
                "to ".green(),
                match final_action.slot.clone().unwrap().as_str() {
                    "active" => final_action.slot.unwrap().green(),
                    "inactive" => final_action.slot.unwrap().blue(),
                    _ => final_action.slot.unwrap().red(),
                }
                .bold(),
                " slot)".green(),
            ),
            Action::Drop => format!(
                "{}{}{}{}{}{}{}",
                " (".green(),
                final_action.file.unwrap().bold(),
                ", from ".green(),
                match final_action.slot.clone().unwrap().as_str() {
                    "active" => final_action.slot.unwrap().green(),
                    "inactive" => final_action.slot.unwrap().blue(),
                    _ => final_action.slot.unwrap().red(),
                }
                .bold(),
                " slot".green(),
                match parse_config()?.history_display_dir {
                    true => format!(
                        " {} {}",
                        "to".green(),
                        final_action.path.unwrap().to_str().unwrap(),
                    ),
                    false => String::new(),
                },
                ")".green(),
            ),
            _ => String::from(""),
        }
    );

    Ok(())
}

/// Redoes actions made by Vento using the history database located on the Vento directory
pub fn redo(steps: usize) -> Result<()> {
    let path: PathBuf = [
        env_config()?.vento_dir,
        Path::new("history.db3").to_path_buf(),
    ]
    .iter()
    .collect();
    let db = Connection::open(path)?;

    // Determine if step amount is greater than the position of the action
    let mut current = db.prepare("SELECT id FROM history WHERE current = 1")?;
    let actions = current.query_map([], |row| row.get(0))?;
    let last_action: usize = actions.last().unwrap_or(Ok(0))?;

    // Determine table size
    let mut size_transaction = db.prepare("SELECT id FROM history")?;
    let size_actions = size_transaction.query_map([], |row| row.get(0))?;
    let size: usize = size_actions.last().unwrap_or(Ok(0))?;

    if size - last_action < steps {
        throw_error(ErrorType::InvalidStepsLength)?;
    }

    let final_dest = last_action + steps;

    // Calculates how many actions need to be redone
    let mut redo_queue_transaction = db.prepare(
	    "SELECT id, path, file, slot, action FROM history WHERE id > ?1 AND id <= ?2 ORDER BY id ASC",
	)?;
    let redo_queue = redo_queue_transaction.query_map([last_action, final_dest], |row| {
        Ok(HistoryData {
            id: row.get(0)?,
            path: Some(PathBuf::from(row.get::<_, String>(1)?)),
            file: row.get(2)?,
            slot: row.get(3)?,
            action: match row.get::<_, String>(4)?.as_str() {
                "take" => Action::Take,
                "drop" => Action::Drop,
                "switch" => Action::Switch,
                _ => unreachable!(),
            },
            time: 0,
            current: 0,
        })
    })?;

    // Redoes actions for each step
    for raw_step in redo_queue {
        let step = raw_step?;

        match step.action {
            Action::Take => {
                let path: String = [
                    String::from(step.path.unwrap().to_str().unwrap()),
                    step.file.unwrap(),
                ]
                .join("/");
                item::take(&path, step.slot.unwrap().as_str(), false, false, false)?;
            }
            Action::Drop => {
                item::drop(
                    &step.file.unwrap(),
                    &step.slot.unwrap(),
                    step.path.unwrap(),
                    false,
                    false,
                    false,
                )?;
            }
            Action::Switch => inv::switch(false, false)?,
        }

        db.execute("UPDATE history SET current = 0 WHERE current = 1", ())?;
        db.execute("UPDATE history SET current = 1 WHERE id = ?1", [step.id])?;
    }

    // Prepares to display details of the final position
    let mut final_transaction = db.prepare("SELECT * FROM history WHERE current = 1")?;
    let final_action_iter = final_transaction.query_map([], |row| {
        Ok(HistoryData {
            id: row.get(0)?,
            path: Some(PathBuf::from(row.get::<_, String>(1)?)),
            file: row.get(2)?,
            slot: row.get(3)?,
            action: match row.get::<_, String>(4)?.as_str() {
                "take" => Action::Take,
                "drop" => Action::Drop,
                "switch" => Action::Switch,
                _ => unreachable!(),
            },
            time: row.get(5)?,
            current: row.get::<_, i32>(5)?,
        })
    })?;

    let final_action = final_action_iter.last().unwrap()?;

    // Formats the current action's timestamp to readable, local time
    let timestamp = final_action.time;
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0);
    let datetime = TimeZone::from_utc_datetime(&Local, &naive.unwrap());
    let newdate = datetime.format("%Y-%m-%d, %H:%M:%S");

    // Prints transaction result
    println!(
        "{}{}{}{}{}{}",
        append_emoji(EmojiType::Success)?,
        "Returned to ".green(),
        match final_action.action {
            Action::Take => "Take",
            Action::Drop => "Drop",
            Action::Switch => "Switch",
        }
        .bold(),
        " action, on ".green(),
        newdate,
        match final_action.action {
            Action::Take => format!(
                "{}{}{}{}{}{}{}",
                " (".green(),
                final_action.file.unwrap().bold(),
                ", ".green(),
                match parse_config()?.history_display_dir {
                    true => format!(
                        "{} {} ",
                        "from".green(),
                        final_action.path.unwrap().to_str().unwrap(),
                    ),
                    _ => String::new(),
                },
                "to ".green(),
                match final_action.slot.clone().unwrap().as_str() {
                    "active" => final_action.slot.unwrap().green(),
                    "inactive" => final_action.slot.unwrap().blue(),
                    _ => final_action.slot.unwrap().red(),
                }
                .bold(),
                " slot)".green(),
            ),
            Action::Drop => format!(
                "{}{}{}{}{}{}{}",
                " (".green(),
                final_action.file.unwrap().bold(),
                ", from ".green(),
                match final_action.slot.clone().unwrap().as_str() {
                    "active" => final_action.slot.unwrap().green(),
                    "inactive" => final_action.slot.unwrap().blue(),
                    _ => final_action.slot.unwrap().red(),
                }
                .bold(),
                " slot".green(),
                match parse_config()?.history_display_dir {
                    true => format!(
                        " {} {}",
                        "to".green(),
                        final_action.path.unwrap().to_str().unwrap(),
                    ),
                    false => String::new(),
                },
                ")".green(),
            ),
            _ => String::from(""),
        }
    );

    Ok(())
}

/// Displays n actions before and after the current action
pub fn view(length: isize) -> Result<()> {
    let path: PathBuf = [
        env_config()?.vento_dir,
        Path::new("history.db3").to_path_buf(),
    ]
    .iter()
    .collect();
    let db = Connection::open(path)?;

    // Determine table size
    let mut size_transaction = db.prepare("SELECT id FROM history")?;
    let size_actions = size_transaction.query_map([], |row| row.get(0))?;
    let size: isize = size_actions.last().unwrap_or(Ok(0))?;

    let (x, _) = termion::terminal_size().unwrap();

    // If there's no history, don't print the table
    if size == 0 {
        println!(
            "{}{}",
            append_emoji(EmojiType::Success)?,
            "No data to show".green()
        );
    }

    // Find last action
    let mut current = db.prepare("SELECT id FROM history WHERE current = 1")?;
    let actions = current.query_map([], |row| row.get(0))?;
    let last_action: isize = actions.last().unwrap_or(Ok(0))?;

    let mut forward: isize = last_action + length;
    let mut backward: isize = last_action - length;
    let total_range: isize = length * 2;

    // Changes ranges in case they exceed the table margins
    if forward >= size {
        forward = size;
        backward = size - total_range;
    } else if backward < 1 {
        backward = 1;
        forward = total_range + 1;
    }

    // Read from table
    let mut history_transaction =
        db.prepare("SELECT * FROM history WHERE id >= ?1 AND id <= ?2")?;
    let history = history_transaction.query_map([backward, forward], |row| {
        Ok(HistoryData {
            id: row.get(0)?,
            path: Some(PathBuf::from(row.get::<_, String>(1)?)),
            file: row.get(2)?,
            slot: row.get(3)?,
            action: match row.get::<_, String>(4)?.as_str() {
                "take" => Action::Take,
                "drop" => Action::Drop,
                "switch" => Action::Switch,
                _ => unreachable!(),
            },
            time: row.get(5)?,
            current: row.get(6)?,
        })
    })?;

    // Terminal needs to be at least 83 columns wide
    if x < 83 {
        throw_error(ErrorType::SmallTerminal)?;
    }

    let mut space_left: usize = (x - 83).into();

    // Append separators to ID
    let mut id_separators = String::new();
    if size.to_string().len() > 2 {
        for _ in 0..size.to_string().len() - 2 {
            id_separators.insert(id_separators.len(), '-')
        }
        space_left = space_left - size.to_string().len() + 2;
    }

    // Append separators to path column
    let mut path_separators = String::new();
    let mut file_separators = String::new();

    // Calculate spaces left to add padding to the path and file separators
    space_left /= 3;
    for _ in 0..space_left * 2 {
        path_separators.insert(path_separators.len(), '-')
    }
    for _ in 0..space_left {
        file_separators.insert(file_separators.len(), '-')
    }

    let separator = format!(
        "+----{}+---------------------+--------+------------------{}+----------{}+----------+---+",
        id_separators, path_separators, file_separators
    );

    // Render the first column names
    println!("{}", separator);
    print!("| ");
    if size.to_string().len() > 2 {
        for _ in 0..size.to_string().len() - 2 {
            print!(" ")
        }
    }
    print!("ID | Date                | Action | Path             ");
    for _ in 0..space_left * 2 {
        print!(" ")
    }
    print!("| File     ");
    for _ in 0..space_left {
        print!(" ")
    }
    println!("| Slot     | C |\n{}", separator);

    // Print the rows
    for raw_step in history {
        let step = raw_step?;

        // Format timestamp on row
        let timestamp = step.time;
        let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0);
        let datetime = TimeZone::from_utc_datetime(&Local, &naive.unwrap());
        let fdate = datetime.format("%Y-%m-%d %H:%M:%S");

        // Add spacing for ID column
        let mut id_pad = String::new();
        let id = step.id.to_string().len();

        if size.to_string().len() >= 2 {
            let id_pad_len = size.to_string().len();
            for x in 0..id_pad_len - id {
                id_pad.insert(x, ' ');
            }
        } else {
            id_pad.insert(0, ' ');
        }

        // Add spacing to fit inside the file column
        let file_len = match &step.file {
            Some(x) => x.len(),
            None => 0,
        };
        let mut file_pad = String::new();
        let mut file = step.file.unwrap_or(String::from(""));
        let file_column_len;
        if file_len > space_left + 8 {
            file_column_len = 0;
            let mut reversed: String = file.chars().rev().collect();
            for _ in 0..file_len - space_left - 5 {
                reversed.pop();
            }
            file = reversed.chars().rev().collect();
            for x in 0..3 {
                file.insert(x, '.');
            }
        } else {
            file_column_len = space_left + 8 - file_len
        }

        for x in 0..file_column_len {
            file_pad.insert(x, ' ');
        }

        // Add spacing to fit inside the path column
        let mut path_pad = String::new();
        let mut path = step
            .path
            .unwrap_or(PathBuf::new())
            .to_string_lossy()
            .to_string();
        let path_len = path.len();
        let path_column_len;
        if path_len > space_left * 2 + 16 {
            path_column_len = 0;
            let mut reversed: String = path.chars().rev().collect();
            for _ in 0..path_len - space_left * 2 - 13 {
                reversed.pop();
            }
            path = reversed.chars().rev().collect();
            for x in 0..3 {
                path.insert(x, '.');
            }
        } else {
            path_column_len = space_left * 2 + 16 - path_len;
        }

        for _ in 0..path_column_len {
            path_pad.insert(path_pad.len(), ' ');
        }

        // Add spacing on slot column
        let mut slot = step.slot.unwrap_or(String::from(""));
        if slot == "active" {
            slot = String::from("active  ");
        } else if slot == "inactive" {
            slot = String::from("inactive");
        } else {
            slot = String::from("        ")
        }

        println!(
            "| {}{} | {} | {} | {}{} | {}{} | {} | {} |",
            id_pad,
            step.id,
            fdate,
            match step.action {
                Action::Take => "Take  ",
                Action::Drop => "Drop  ",
                Action::Switch => "Switch",
            },
            path,
            path_pad,
            file,
            file_pad,
            slot,
            match step.current {
                0 => " ",
                1 => "*",
                _ => " ",
            }
        );
    }
    println!("{}", separator);

    Ok(())
}

/// Migrate old "last" file into the history database
pub fn migrate() -> Result<()> {
    // Get last file from previous location
    let last_path: PathBuf = [env_config()?.vento_dir, Path::new("last").to_path_buf()]
        .iter()
        .collect();

    if !last_path.is_file() {
        throw_error(ErrorType::NoFileOrDir)?;
    }

    let last_file = fs::read_to_string(&last_path)?;

    let mut contents = vec![];

    for line in last_file.lines() {
        contents.push(line);
    }

    if contents.len() != 4 {
        throw_error(ErrorType::InvalidHistoryLength)?;
    }

    // Write contents of file into history database
    common::history(HistoryData {
        id: 0,
        path: Some(Path::new(contents[0]).to_path_buf()),
        file: Some(String::from(contents[1])),
        slot: Some(String::from(contents[2])),
        action: match contents[3] {
            "take" => Action::Take,
            "drop" => Action::Drop,
            "switch" => Action::Switch,
            _ => unreachable!(),
        },
        time: 0,
        current: 1,
    })?;

    fs::remove_file(last_path)?;

    println!(
        "{}{}",
        append_emoji(EmojiType::Success)?,
        "Migrated history file to database".green()
    );

    Ok(())
}
