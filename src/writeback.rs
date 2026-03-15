use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::config::Config;
use crate::discovery::discover_book;
use crate::render::render_validated_summary;

pub fn build_summary(book_root: &Path, config: &Config) -> Result<String> {
    let tree = discover_book(book_root, config)?;
    render_validated_summary(&tree)
}

pub fn summary_output_path(book_root: &Path, config: &Config) -> PathBuf {
    let configured_path = Path::new(&config.summary_path);
    if configured_path.is_absolute() {
        configured_path.to_path_buf()
    } else {
        book_root.join(&config.book_src).join(configured_path)
    }
}

pub fn read_summary(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))
}

pub fn write_summary(book_root: &Path, config: &Config, summary: &str) -> Result<()> {
    let output_path = summary_output_path(book_root, config);
    fs::write(&output_path, summary)
        .with_context(|| format!("Failed to write {}", output_path.display()))
}

pub fn diff_summaries(current: &str, generated: &str) -> String {
    format!("--- current\n+++ generated\n- {current}\n+ {generated}")
}
