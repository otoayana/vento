![Vento](https://codeberg.org/nixgoat/vento/media/branch/master/assets/logo.png "Vento")

[![Latest version](https://shields.io/crates/v/vento?color=red)](https://crates.io/crates/vento)
[![Downloads](https://shields.io/crates/d/vento)](https://crates.io/crates/vento)
[![Licensed under GPLv3](https://shields.io/crates/l/vento)](https://codeberg.org/nixgoat/vento/src/branch/master/LICENSE.md)

Vento is a utility which allows you to manage your files as if you're playing an old text adventure. It's made in Rust and originally inspired by [Chesapeake's Inventory](https://github.com/mothdotmonster/inventory).

## Installation

### 1) Cargo (Recommended)

Make sure Rust is installed, along with `cargo`, Rust's package manager.

```
$ cargo install vento
```

### 2) Manually

Clone the repository using `git`.

```
$ git clone https://codeberg.org/nixgoat/vento.git && cd vento
```

### 2.a) cargo-make

This install method additionally installs the manpages for Vento. Make sure Rust, `cargo` and `cargo-make` are installed.

```
$ cargo make install
```

### 2.b) Cargo

Make sure Rust is installed, along with `cargo`, Rust's package manager.

```
$ cargo install --path .
```

## Quick Start

After installing, run:

```
$ vento -i
```

This will create a `.vento` folder in your home directory, which will store your inventories. Some basic commands include:

```
// listing files in the currently active inventory
$ vento

// switching inventory slots
$ vento -c

// taking a file or directory
$ take <file|directory>

// dropping a file or directory
$ drop <file|directory> [destination]
```

For additional documentation, you can check the documentation for each command by running the following.

```
$ (command) -h
```

Or, if Vento was installed through `cargo-make`, check the manpages by running:

```
$ man (command)
```

## Credits

- [Chesapeake](https://moth.monster/) for the original concept
- [jo!](https://codeberg.org/j0) for helping me with Rust concepts!
