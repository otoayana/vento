![Vento](assets/logo.png "Vento")

Vento is a utility which allows you to manage your files as if you're playing an old text adventure. It's made in Rust and originally inspired by [Chesapeake's Inventory](https://github.com/mothdotmonster/inventory).

## Install

Clone the repository using Git.

```
$ git clone https://codeberg.org/nixgoat/vento.git && cd vento
```

The recommended method to install Vento is to use [cargo-make](https://crates.io/crates/cargo-make/0.3.54#usage-conditions-structure). This will install the binary and the manpages for Vento.

```
$ cargo make install
```

Otherwise you can build and install it with [Cargo](https://rustup.rs/). This will however not install the manpages.

```
$ cargo install --path .
```

## Quickstart

After installing, run `vento -i`. This will create a `.vento` folder in your home directory which will store your inventories. After which, you can run `vento` to display the files in your inventories, `take` to move a file into your active inventory and `drop` to drop a file out of it. If you're stuck, run `vento -h` or check the manpage by running `man vento`.

## Credits

- [Chesapeake](https://moth.monster/) for the original concept
- [jo!](https://codeberg.org/j0) for helping me with Rust concepts!
