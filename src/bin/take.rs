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
use std::env;
use vento::{help, item};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        if args[1].contains("--slot=") {
            match args.len() {
                3 => item::take(&args[2], &args[1].replace("--slot=", ""))?,
                2 => bail!("❌ {}", "You need to specify a file".red()),
                _ => bail!("❌ {}", "Too many arguments".red()),
            };
        } else {
            match args[1].as_str() {
                "--help" | "-h" => help::take()?,
                "-s" => match args.len() {
                    4 => item::take(&args[3], &args[2])?,
                    3 => bail!("❌ {}", "You need to specify a file".red()),
                    2 => bail!("❌ {}", "You need to specify a slot".red()),
                    _ => bail!("❌ {}", "Too many arguments".red()),
                },
                _ => match args.len() {
                    2 => item::take(&args[1], &String::from("active"))?,
                    _ => bail!("❌ {}", "Too many arguments".red()),
                },
            }
        }
    } else {
        help::take()?;
    }
    Ok(())
}
