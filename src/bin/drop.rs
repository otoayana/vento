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
use std::path::PathBuf;
use vento::{common::get_current_dir, item};

#[derive(Parser)]
#[command(name = "Drop")]
#[command(about = "A file dropper for Vento", long_about = None)]
#[command(author, version)]
struct Cli {
    /// Pick a slot to drop the file from
    #[arg(short, long)]
    slot: Option<String>,

    /// File to drop from inventory
    file: String,
    /// Location to drop file onto
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    // Handles args in Drop
    let cli = Cli::parse();
    let unwrapped_slot = cli.slot.unwrap_or(String::from("active"));
    let slot = unwrapped_slot.as_str();
    let out = cli.output.unwrap_or(get_current_dir()?);

    item::drop(&cli.file, slot, out, true)?;

    Ok(())
}
