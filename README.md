# libcangjie-howtotype

Find out how to type a character by querying libcangjie's database.

This repository contains a CLI tool and a Rust library.

## CLI Usage

### CLI Synopsis

```
Find out how to type a character by querying libcangjie's database

Usage: libcangjie-howtotype [OPTIONS] <CHARACTER>

Arguments:
  <CHARACTER>  The character to query

Options:
  -C, --cj-version <VERSION>   The version of Cangjie used [default: 3] [possible values: 3, 5]
  -f, --format <FORMAT>        The output format [default: radical] [possible values: code, radical]
  -s, --separator <SEPARATOR>  The separator between codes [default: "\n"]
  -q, --quiet                  Do not report an error when the command doesn't know how to type
  -h, --help                   Print help
  -V, --version                Print version
```

### CLI Examples

```sh
libcangjie-howtotype 喵 # 口廿田
```

## Library Usage

```rust
use libcangjie_howtotype::{CangjieCode, CangjieVersion, LibCangjieHowToType};

let cangjie = LibCangjieHowToType::new()?;

let how_to_type = cangjie.how_to_type("喵", CangjieVersion::V3)?;
println!("{}", how_to_type[0].radicals()); // 口廿田
```
