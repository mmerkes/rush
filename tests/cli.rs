use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn test_echo() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rush")?;
    cmd.arg("echo").arg("some string");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("some string"));

    Ok(())
}

#[test]
fn test_ls() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rush")?;
    cmd.arg("ls").arg("tests");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("cli.rs"));

    Ok(())
}