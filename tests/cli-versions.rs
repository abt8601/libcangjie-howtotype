use std::error::Error;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn test_cli_v3() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("libcangjie-howtotype")?;

    cmd.arg("-C").arg("3").arg("屬");
    cmd.assert().success().stdout(predicate::eq("尸卜卜戈\n"));

    Ok(())
}

#[test]
fn test_cli_v5() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("libcangjie-howtotype")?;

    cmd.arg("-C").arg("5").arg("屬");
    cmd.assert().success().stdout(predicate::eq("尸水田戈\n"));

    Ok(())
}
