use mdbook_summary_tools::{
    config::load_config, discovery::discover_book, render::render_summary,
};

#[test]
fn renderer_emits_homepage_once_then_nested_sections() {
    let cfg = load_config("tests/fixtures/render/book.toml").unwrap();
    let tree = discover_book("tests/fixtures/render", &cfg).unwrap();
    let summary = render_summary(&tree);
    let expected =
        "[wiki.ashish.me](<readme.md>)\n- [Git](<git/readme.md>)\n  - [Rebase](<git/rebase.md>)\n";

    assert_eq!(summary, expected);
}
