use mdbook_generate_summary::config::load_config;

#[test]
fn native_config_overrides_autosummary_compat_values() {
    let cfg = load_config("tests/fixtures/config/book.toml").unwrap();

    assert_eq!(
        cfg.index_names,
        vec!["readme.md".to_string(), "index.md".to_string()]
    );
    assert_eq!(cfg.homepage.as_ref().unwrap().path, "readme.md");
    assert_eq!(cfg.book_src, ".".to_string());
    assert!(!cfg.ignore_hidden);
}
