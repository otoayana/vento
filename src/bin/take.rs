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
use clap::Parser;
use vento::{common::override_color, item};

#[derive(Parser)]
#[command(name = "Take")]
#[command(about = "A file grabber for Vento", long_about = None)]
#[command(author, version)]
struct Cli {
    /// Pick a slot to take the file into
    #[arg(short, long)]
    slot: Option<String>,

    /// File to take
    file: String,
}

fn main() -> Result<()> {
    // Handles args in Vento
    override_color()?;
    let cli = Cli::parse();
    let slot = cli.slot.clone().unwrap_or(String::from("active"));

    item::take(&cli.file, &slot, true, cli.slot.is_some())?;
    Ok(())
}
