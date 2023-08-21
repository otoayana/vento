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
use vento::{
    archive,
    common::override_color,
    history, inv,
    message::{throw_error, ErrorType},
};

#[derive(Parser)]
#[command(name = "Vento")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Pick slot to list
    #[arg(short, long)]
    slot: Option<String>,

    /// Switch slots
    #[arg(short = 'c', long)]
    switch: bool,

    /// Undo the last action
    #[arg(short, long)]
    undo: bool,

    /// Export an inventory
    #[arg(short, long, value_names = &["SLOT", "ARCHIVE"], num_args = ..=2)]
    export_inv: Option<Vec<String>>,

    /// Export the Vento directory
    #[arg(short = 'E', long, default_missing_value = "vento.tar.xz", value_name = "ARCHIVE", num_args = ..=1)]
    export_dir: Option<PathBuf>,

    /// Import an inventory archive
    #[arg(short = 'g', long, num_args = 1..=2, value_names = &["ARCHIVE", "SLOT"])]
    import_inv: Option<Vec<String>>,

    /// Import a Vento directory archive
    #[arg(short = 'G', long, value_name = "ARCHIVE")]
    import_dir: Option<PathBuf>,

    /// Initialize Vento
    #[arg(short, long)]
    init: bool,

    directory: Option<String>,
}

fn main() -> Result<()> {
    override_color()?;
    let cli = Cli::parse();
    let unwrapped_dir = cli.directory.unwrap_or(String::new());
    let dir = unwrapped_dir.as_str();

    if cli.switch {
        inv::switch(true)?
    } else if cli.undo {
        history::undo()?
    } else if cli.init {
        inv::init()?
    } else if cli.export_inv.is_some() {
        let unwrapped_export_inv = cli.export_inv.unwrap();
        let export_inv_values = match unwrapped_export_inv.len() {
            0 => vec![String::from("active"), String::from("active.tar.xz")],
            _ => unwrapped_export_inv,
        };

        archive::export_inv(
            match export_inv_values[0].as_str() {
                "active" | "inactive" | "a" | "i" => export_inv_values[0].as_str(),
                _ => "active",
            },
            PathBuf::from(match export_inv_values[0].as_str() {
                "active" | "inactive" | "a" | "i" => export_inv_values[1].as_str(),
                _ => export_inv_values[0].as_str(),
            }),
            true,
        )?
    } else if cli.export_dir.is_some() {
        archive::export_dir(cli.export_dir.unwrap(), true)?
    } else if cli.import_inv.is_some() {
        let import_inv_values = &cli
            .import_inv
            .unwrap_or(vec![String::new(), String::from("active")]);

        match import_inv_values[0].as_str() {
            "" | "active" | "inactive" | "a" | "i" => throw_error(ErrorType::SpecifyFile)?,
            _ => archive::import_inv(
                PathBuf::from(&import_inv_values[0]),
                match import_inv_values.len() {
                    2 => match import_inv_values[1].as_str() {
                        "active" | "inactive" | "a" | "i" => import_inv_values[1].as_str(),
                        _ => "active",
                    },
                    _ => "active",
                },
                true,
            )?,
        };
    } else if cli.import_dir.is_some() {
        archive::import_dir(cli.import_dir.unwrap(), true)?
    } else {
        inv::list(cli.slot.unwrap_or(String::from("active")).as_str(), dir)?
    }

    Ok(())
}
