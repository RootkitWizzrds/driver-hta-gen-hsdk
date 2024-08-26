use std::process::Command;

#[test]
fn test_integration() {
    let output = Command::new("./target/release/hta-gen")
        .arg("output.hta")
        .arg("driver.bin")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Integration test failed");
}
