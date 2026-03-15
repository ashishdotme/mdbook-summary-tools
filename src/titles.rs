use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

pub fn resolve_title(path: impl AsRef<Path>) -> Result<String> {
    let default_order = [
        "frontmatter".to_string(),
        "h1".to_string(),
        "filename".to_string(),
    ];
    resolve_title_with_sources(path, &default_order)
}

pub fn resolve_title_with_sources(path: impl AsRef<Path>, sources: &[String]) -> Result<String> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path)
        .with_context(|| format!("Failed to read markdown file {}", path.display()))?;

    for source in sources {
        match source.as_str() {
            "frontmatter" => {
                if let Some(title) = frontmatter_title(&contents) {
                    return Ok(title);
                }
            }
            "h1" => {
                if let Some(title) = h1_title(&contents) {
                    return Ok(title);
                }
            }
            "filename" => {
                return Ok(filename_title(path));
            }
            _ => {}
        }
    }

    Ok(filename_title(path))
}

fn frontmatter_title(contents: &str) -> Option<String> {
    let mut lines = contents.lines();
    if lines.next()? != "---" {
        return None;
    }

    for line in lines {
        if line == "---" {
            break;
        }
        if let Some(value) = line.strip_prefix("title:") {
            return Some(value.trim().trim_matches('"').to_string());
        }
    }

    None
}

fn h1_title(contents: &str) -> Option<String> {
    contents.lines().find_map(|line| {
        line.strip_prefix("# ")
            .map(str::trim)
            .filter(|title| !title.is_empty())
            .map(ToOwned::to_owned)
    })
}

fn filename_title(path: &Path) -> String {
    path.file_stem()
        .and_then(|segment| segment.to_str())
        .unwrap_or_default()
        .to_string()
}
