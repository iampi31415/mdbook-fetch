/// Use Command to run the binary with specific content.
use std::process::Command;

static BIN_PATH: &'static str = env!("CARGO_BIN_EXE_mdbook-fetch");
#[test]
fn bin_test_supports() {
    let output = Command::new(BIN_PATH)
        .args(["supports", "html"])
        .output()
        .expect("failed to execute process");

    assert_eq!(output.status.code().unwrap(), 0);
}

#[test]
fn bin_test_no_supports() {
    let output = Command::new(BIN_PATH)
        .args(["supports", "hsxl"])
        .output()
        .expect("failed to execute process");
    assert_eq!(output.status.code().unwrap(), 1);
}
#[test]
fn bin_test_wrong_input() {
    let output = Command::new(BIN_PATH)
        .args(["something", "else"])
        .output()
        .expect("failed to execute process");
    assert_eq!(output.status.code().unwrap(), 1);
}
