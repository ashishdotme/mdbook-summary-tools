use mdbook_summary_tools::validate::validate_summary;

#[test]
fn validator_rejects_invalid_summary_output() {
    let result = validate_summary("- [Broken](<broken.md>\n");

    assert!(result.is_err());
}
