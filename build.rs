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
use man::prelude::*;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::Write;

fn main() -> Result<()> {
    if cfg!(unix) {
        let pages = [vento()?, take()?, drop()?];

        let tempdir = env::temp_dir().join("vento-man");

        create_dir_all(tempdir.clone())?;

        for page in 0..pages.len() {
            let tempfile = tempdir.join(pages[page].clone().1);
            let mut file = File::create(tempfile).unwrap();
            write!(&mut file, "{}", pages[page].clone().0).unwrap();
        }
    }
    Ok(())
}

fn vento() -> Result<(String, String)> {
    let page = Manual::new("vento")
        .about("a CLI inventory for your files")
        .author(Author::new("Lux Aliaga").email("they@mint.lgbt"))
        .description("List files and directories in the currently active inventory, the files in SLOT, the files in DIRECTORY or the files in DIRECTORY in SLOT.")
        .flag(
            Flag::new()
                .short("-s")
                .long("--switch")
                .help("Switches inventory slots"),
        )
        .flag(
            Flag::new()
                .short("-i")
                .long("--init")
                .help("Initializes Vento with all its respective directories"),
        )
        .flag(
            Flag::new()
                .short("-h")
                .long("--help")
                .help("Shows the help message"),
        )
        .arg(Arg::new("[SLOT]"))
        .arg(Arg::new("[DIRECTORY]"))
        .custom(
            Section::new("before starting")
            .paragraph("Vento will first need to initialize the respective directories before usage. Do this by running vento -i.")
        )
        .render();

    Ok((page, String::from("vento.1")))
}

fn take() -> Result<(String, String)> {
    let page = Manual::new("take")
        .about("a file grabber for Vento")
        .author(Author::new("Lux Aliaga").email("they@mint.lgbt"))
        .description("Take FILE and put it in the inventory.")
        .option(
            Opt::new("slot")
                .short("-s")
                .long("--slot")
                .help("The slot to put the file in"),
        )
        .arg(Arg::new("FILE"))
        .render();

    Ok((page, String::from("take.1")))
}

fn drop() -> Result<(String, String)> {
    let page = Manual::new("drop")
        .about("a file dropper for Vento")
        .author(Author::new("Lux Aliaga").email("they@mint.lgbt"))
        .description("Take FILE off the inventory and drop it in DESTINATION.")
        .option(
            Opt::new("slot")
                .short("-s")
                .long("--slot")
                .help("The slot to take the file from"),
        )
        .arg(Arg::new("FILE"))
        .arg(Arg::new("[DESTINATION]"))
        .render();

    Ok((page, String::from("drop.1")))
}
