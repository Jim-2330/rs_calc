# rs_calc
> A small calculator made using rust

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/Jim-2330/rs_calc/actions/workflows/rust.yml/badge.svg)](https://github.com/Jim-2330/rs_calc/actions/workflows/rust.yml)
[![GitHub Release](https://img.shields.io/github/v/release/Jim-2330/rs_calc?include_releases&style=flat-square)](https://github.com/Jim-2330/rs_calc/releases)
[![Rust](https://img.shields.io/badge/rust-1.85.0%2B-blue.svg?maxAge=3600)](https://github.com/rs_calc)

---

## Features

- **Parse Arguments:** Parse expressions and options directly from the command line.
- **BigDecimal:** High-precision calculation to solve classic floating-point issues like `0.1 + 0.2 != 0.3`.
- **REPL & Analysis:** Supports multiple modes (`eval`, `tokens`, `ast`) for expression evaluation and inspection.

---

## Prerequisites

Before running this project please install rust that depandency

- rustc >= 1.99.1


---

## Quick Start

### First method
1. go on the [release page](https://github.com/Jim-2330/rs_calc/releases)
2. choose the correct OS and download
3. unzip it
4. if you are using macosy ou need to 
``` bash
cd ~/Downloads
xattr -d com.apple.quarantine rs_calc
chmod +x rs_calc
```

### Second method
If you want to use this method you need prerequirement
``` bash
cargo install rs_calc
cargo build --release
```

---

## Demo to use
``` bash
$ rs_calc -h
a tiny calculator

Usage: rs_calc.exe [OPTIONS] [EXPR]

Arguments:
  [EXPR]

Options:
  -m, --mode <MODE>  [default: eval] [possible values: eval, tokens, ast]
  -h, --help         Print help
  -V, --version      Print version
$ rs_calc -m eval "1 + 1"
2
```

---

## Contribute with us
Welcome to pull request and give us issue！
1. fork the project
2. create your own fork of feature
3. commit your own change
4. push your own fork
5. add a new pull request

---

## license

This project uses [MIT license](LICENSE.txt) to open source
