#[test]
fn check_mode_fails_when_summary_drift_exists() {
    let fixture_dir = std::path::Path::new("tests/fixtures/writeback");

    let mut cmd = assert_cmd::Command::cargo_bin("mdbook-generate-summary").unwrap();
    cmd.current_dir(fixture_dir).arg("check").assert().failure();
}
