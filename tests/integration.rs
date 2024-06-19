#![allow(unused_variables)]

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
pub fn test_help(){

    let help = Command::cargo_bin("rmconvert")
        .unwrap()
        .arg("-h")
        .assert()
        .success();

    let help_txt = String::from_utf8(help.get_output().stdout.to_owned()).unwrap();
    let help_pred = predicate::str::starts_with(&help_txt);

    Command::cargo_bin("rmconvert")
        .unwrap()
        .assert()
        .failure()
        .stderr(help_pred);
}

