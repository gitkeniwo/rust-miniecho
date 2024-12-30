use assert_cmd::Command; 
use std::fs;
use predicates::prelude::*;

// Test functions must be free, monomorphic functions that take no arguments 
// and return a type implementing the `Termination` trait (e.g., (), Result<T, E>, or !)
// See https://doc.rust-lang.org/reference/attributes/testing.html
type TestResult = Result<(), Box<dyn std::error::Error>>;

fn runs(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?; 

    Command::cargo_bin("miniecho")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn works() -> TestResult {
    let mut cmd = Command::cargo_bin("miniecho")?;
    cmd.arg("hello")
        .assert()
        .success(); 
    Ok(())
}

/// miniecho dies when no args are provided and raises stderr
#[test] 
fn dies_no_args() -> TestResult{ 
    let mut cmd = Command::cargo_bin("miniecho")?; 
    cmd.assert() 
        .failure() 
        .stderr(predicate::str::contains("the following required arguments were not provided"));
    Ok(()) 
}

#[test] 
fn dies_invalid_args() -> TestResult{ 
    let mut cmd = Command::cargo_bin("miniecho")?; 
    cmd.arg("--something")   
        .assert() 
        .failure() 
        .stderr(predicate::str::contains("unexpected argument")); 
    Ok(())
}

#[test] 
fn hello1() -> TestResult {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile)?; 

    let mut cmd = Command::cargo_bin("miniecho")?; 
    cmd.arg("Hello there").assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn hello2() -> TestResult{
    runs(&vec!["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1n() -> TestResult{
    runs(&vec!["-n", "Hello there"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2n() -> TestResult{
    runs(&vec!["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}