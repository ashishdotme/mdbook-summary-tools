#[test]
fn e2e_generate_creates_expected_summary_for_realistic_book() {
    let fixture_dir = std::path::Path::new("tests/fixtures/e2e");

    let output = assert_cmd::Command::cargo_bin("mdbook-summary-tools")
        .unwrap()
        .current_dir(fixture_dir)
        .args(["generate", "--stdout"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let output = String::from_utf8(output).unwrap();

    insta::assert_snapshot!(output, @r#"
    [wiki.ashish.me](<readme.md>)
    - [Clean Code](<clean-code/readme.md>)
      - [Conventional Commits](<clean-code/conventional-commits.md>)
    "#);
}
