# 🪛 oxips

[![Rust](https://github.com/orellazri/oxips/actions/workflows/rust.yml/badge.svg)](https://github.com/orellazri/oxips/actions/workflows/rust.yml)

oxips is a simple IPS binary patching tool written in Rust. It is useful, for example, when patching video game roms.

## Build

Clone the repository and build using:

```bash
cargo build --release
```

## Usage

```bash
oxips --rom <rom file> --patch <patch file> --output <output file>
```
