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
use colored::Colorize;

pub fn vento() -> Result<()> {
    // A quick guide to move around in Vento
    println!(
        "{}, a CLI inventory for your files
© 2022 Lux Aliaga. Licensed under GPLv3

{}
    - {}: Lists files in selected inventory
    - {}: Switches slots
    - {}: Initializes Vento
    - {}: Displays this message",
        "Vento".bold().blue(),
        "Usage:".bold(),
        "vento [ -s slot | --slot=slot ] [ directory ]"
            .bold()
            .green(),
        "vento ( -c | --switch )".bold().green(),
        "vento ( -i | --init )".bold().green(),
        "vento ( -h | --help )".bold().green()
    );
    Ok(())
}

pub fn take() -> Result<()> {
    // A quick guide to move around in Take
    println!(
        "{}, a file grabber for Vento
© 2022 Lux Aliaga. Licensed under GPLv3

{}
    - {}: Takes a file and saves it in the inventory
    - {}: Displays this message",
        "Take".bold().blue(),
        "Usage:".bold(),
        "take [ -s slot | --slot=slot ] file | directory"
            .bold()
            .green(),
        "take ( -h | --help )".bold().green()
    );
    Ok(())
}

pub fn drop() -> Result<()> {
    // A quick guide to move around in Drop
    println!(
        "{}, a file dropper for Vento
© 2022 Lux Aliaga. Licensed under GPLv3

{}
    - {}: Takes a file off the inventory and drops it.
    - {}: Displays this message",
        "Drop".bold().blue(),
        "Usage:".bold(),
        "drop [ -s slot | --slot=slot ] file | directory [destination]"
            .bold()
            .green(),
        "drop ( -h | --help )".bold().green()
    );
    Ok(())
}