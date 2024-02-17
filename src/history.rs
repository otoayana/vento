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
    common::{env_config, parse_config, Action, HistoryData},
    inv, item,
    message::{append_emoji, throw_error, EmojiType, ErrorType},
};
use anyhow::Result;
use chrono::prelude::*;
use colored::Colorize;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

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
    let actions = current.query_map([], |row| Ok(row.get(0)?))?;
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
                let path: String = vec![
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
	let actions = current.query_map([], |row| Ok(row.get(0)?))?;
	let last_action: usize = actions.last().unwrap_or(Ok(0))?;
    
    // Determine table size
	let mut size_transaction = db.prepare("SELECT id FROM history")?;
	let size_actions = size_transaction.query_map([], |row| Ok(row.get(0)?))?;
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
		    let path: String = vec![
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
