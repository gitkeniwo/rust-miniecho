use assert_cmd::Command; 
use std::fs;
use predicates::prelude::*;

#[test] 
fn runs() {
    let mut cmd = Command::cargo_bin("miniecho").unwrap();
    cmd.arg("hello")
        .assert()
        .success(); 
}

/// miniecho dies when no args are provided and raises stderr
#[test] 
fn dies_no_args() { 
    let mut cmd = Command::cargo_bin("miniecho").unwrap(); 
    cmd.assert() 
        .failure() 
        .stderr(predicate::str::contains("the following required arguments were not provided")); 
}

#[test] 
fn dies_invalid_args() { 
    let mut cmd = Command::cargo_bin("miniecho").unwrap(); 
    cmd.arg("--something")   
        .assert() 
        .failure() 
        .stderr(predicate::str::contains("unexpected argument")); 
}

#[test] 
fn hello1() {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap(); 

    let mut cmd = Command::cargo_bin("miniecho").unwrap(); 
    cmd.arg("Hello there").assert()
        .success()
        .stdout(expected);
}