use std::error::Error;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn cli_meow() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("libcangjie-howtotype")?;

    cmd.arg("喵");
    cmd.assert().success().stdout(predicate::eq("口廿田\n"));

    Ok(())
}
