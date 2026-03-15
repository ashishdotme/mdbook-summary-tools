#[test]
fn generate_help_exits_successfully() {
    let mut cmd = assert_cmd::Command::cargo_bin("mdbook-generate-summary").unwrap();
    cmd.arg("--help").assert().success();
}
