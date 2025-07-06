# libcangjie-howtotype

Find out how to type a character by querying libcangjie's database.

This repository contains a CLI tool and a Rust library.

## CLI Usage

TBD

## Library Usage

```rust
use libcangjie_howtotype::{CangjieCode, CangjieVersion, LibCangjieHowToType};

let cangjie = LibCangjieHowToType::new()?;

let how_to_type = cangjie.how_to_type("喵", CangjieVersion::V3)?;
println!("{}", how_to_type[0].radicals()); // 口廿田
```
