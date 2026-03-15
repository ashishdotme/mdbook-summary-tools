use mdbook_summary_tools::{config::load_config, discovery::discover_book};

#[test]
fn discovery_excludes_configured_directories_and_dedupes_homepage() {
    let cfg = load_config("tests/fixtures/discovery/book.toml").unwrap();
    let tree = discover_book("tests/fixtures/discovery", &cfg).unwrap();

    assert!(tree.homepage.is_some());
    assert!(!tree.paths.contains("attachments/ignore.md"));
    assert_eq!(tree.root_markdown_occurrences("readme.md"), 0);
}
