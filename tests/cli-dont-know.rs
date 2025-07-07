use std::error::Error;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn test_cli_dont_know() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("libcangjie-howtotype")?;

    cmd.arg("😀");
    cmd.assert()
        .failure()
        .stdout(predicate::eq(""))
        .stderr(predicate::eq("Error: Don't know how to type '😀'\n"));

    Ok(())
}

#[test]
fn test_cli_dont_know_quiet() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("libcangjie-howtotype")?;

    cmd.arg("-q").arg("😀");
    cmd.assert()
        .success()
        .stdout(predicate::eq(""))
        .stderr(predicate::eq(""));

    Ok(())
}
