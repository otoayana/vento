# Vento

Vento is a utility which allows you to manage your files as if you're playing an old text adventure. It's made in Rust and originally inspired by [Chesapeake's Inventory](https://github.com/mothdotmonster/inventory).

## Install

Clone it using Git and build it with [Cargo](https://rustup.rs/)!

```
$ git clone https://codeberg.org/nixgoat/vento.git && cd vento
$ cargo install --path .
```

## Quickstart

After installing, run `vento init`. This will create a `.vento` folder in your home directory which will store your inventories. After which, you can run `vento list` to display the files in your inventories, `vento take` to move a file into your active inventory and `vento drop` to drop a file out of it. If you're stuck, run `vento help`.

## Credits

- [Chesapeake](https://moth.monster/) for the original concept
