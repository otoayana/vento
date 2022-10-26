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

struct Page {
    content: String,
    file: String,
}

fn main() -> Result<()> {
    if cfg!(unix) {
        let pages = [vento()?, take()?, drop()?, ventotoml()?];

        let tempdir = env::temp_dir().join("vento-man");

        create_dir_all(tempdir.clone())?;

        for page in &pages {
            let tempfile = tempdir.join(&page.file);
            let mut file = File::create(tempfile).unwrap();
            write!(&mut file, "{}", &page.content).unwrap();
        }
    }
    Ok(())
}

fn vento() -> Result<Page> {
    let content = Manual::new("vento")
        .about("a CLI inventory for your files")
        .author(Author::new("Lux Aliaga").email("they@mint.lgbt"))
        .description("List files and directories in the currently active inventory, the files in SLOT, the files in DIRECTORY or the files in DIRECTORY in SLOT.")
        .flag(
            Flag::new()
                .short("-c")
                .long("--switch")
                .help("Switches inventory slots"),
        )
        .flag(
            Flag::new()
                .short("-u")
                .long("--undo")
                .help("Undoes the last action"),
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
        .option(
            Opt::new("slot")
                .short("-s")
                .long("--slot")
                .help("The slot to list"),
        )
        .arg(Arg::new("[DIRECTORY]"))
        .custom(
            Section::new("before starting")
            .paragraph("Vento will first need to initialize the respective directories before usage. Do this by running vento -i.")
        )
        .render();

    Ok(Page {
        content,
        file: String::from("vento.1"),
    })
}

fn take() -> Result<Page> {
    let content = Manual::new("take")
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

    Ok(Page {
        content,
        file: String::from("take.1"),
    })
}

fn drop() -> Result<Page> {
    let content = Manual::new("drop")
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

    Ok(Page {
        content,
        file: String::from("drop.1"),
    })
}

fn ventotoml() -> Result<Page> {
    let content = Manual::new("vento.toml")
        .about("configuration file for Vento")
        .author(Author::new("Lux Aliaga").email("they@mint.lgbt"))
        .description("This is the configuration file for the vento(1), take(1) and drop(1) utilities. Its presence and all its directives are optional.")
        .custom (
            Section::new("supported directives")
            .paragraph("directory = \"PATH\": Changes the path in which Vento's inventories are saved in.")
        )
        .custom (
            Section::new("files")
            .paragraph("Linux: $XDG_CONFIG_HOME/vento.toml")
            .paragraph("macOS: $HOME/Library/Application Support/vento.toml")
            .paragraph("Windows: {FOLDERID_RoamingAppData}\\\\vento.toml")
        )
        .render();

    Ok(Page {
        content,
        file: String::from("vento.toml.1"),
    })
}
