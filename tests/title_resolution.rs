use mdbook_generate_summary::titles::resolve_title;

#[test]
fn title_resolution_uses_frontmatter_then_h1_then_filename() {
    assert_eq!(
        resolve_title("tests/fixtures/titles/frontmatter.md").unwrap(),
        "Frontmatter Title"
    );
    assert_eq!(
        resolve_title("tests/fixtures/titles/heading.md").unwrap(),
        "Heading Title"
    );
    assert_eq!(
        resolve_title("tests/fixtures/titles/fallback-name.md").unwrap(),
        "fallback-name"
    );
}
