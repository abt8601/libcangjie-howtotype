use std::error::Error;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn test_cli_format_code() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("libcangjie-howtotype")?;

    cmd.arg("-f").arg("code").arg("喵");
    cmd.assert().success().stdout(predicate::eq("rtw\n"));

    Ok(())
}

#[test]
fn test_cli_format_radical() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("libcangjie-howtotype")?;

    cmd.arg("-f").arg("radical").arg("喵");
    cmd.assert().success().stdout(predicate::eq("口廿田\n"));

    Ok(())
}
