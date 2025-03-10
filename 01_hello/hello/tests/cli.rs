use std::process::Command;
use assert_cmd::Command as AssertCommand;

#[test]
fn runs() {
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn works() {
    assert!(true);
}

#[test]
fn runs_success() {
    let mut cmd = AssertCommand::cargo_bin("hello").unwrap();
    cmd.assert().success();
}