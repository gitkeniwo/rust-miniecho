

#[test] 
fn hello1() {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap(); 
    let mut cmd = Command::cargo_bin("echor").unwrap(); c
    md.arg("Hello there").assert().success().stdout(expected);
    }