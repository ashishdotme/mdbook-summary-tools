#[test]
fn generate_help_exits_successfully() {
    let mut cmd = assert_cmd::Command::cargo_bin("mdbook-summary-tools").unwrap();
    cmd.arg("--help").assert().success();
}
