use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HomepageConfig {
    pub title: Option<String>,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub book_src: String,
    pub book_title: Option<String>,
    pub summary_path: String,
    pub index_names: Vec<String>,
    pub title_sources: Vec<String>,
    pub sort: String,
    pub directories_first: bool,
    pub ignore_hidden: bool,
    pub exclude: Vec<String>,
    pub include: Vec<String>,
    pub max_depth: usize,
    pub strip_number_prefixes: bool,
    pub marker_mode: String,
    pub homepage: Option<HomepageConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            book_src: "src".to_string(),
            book_title: None,
            summary_path: "SUMMARY.md".to_string(),
            index_names: vec!["index.md".to_string()],
            title_sources: vec![
                "frontmatter".to_string(),
                "h1".to_string(),
                "filename".to_string(),
            ],
            sort: "natural".to_string(),
            directories_first: true,
            ignore_hidden: true,
            exclude: Vec::new(),
            include: Vec::new(),
            max_depth: 0,
            strip_number_prefixes: false,
            marker_mode: "full-file".to_string(),
            homepage: None,
        }
    }
}

pub fn resolve_config_path(
    book_root: Option<&Path>,
    config_path: Option<&Path>,
) -> Result<PathBuf> {
    if let Some(config_path) = config_path {
        return Ok(config_path.to_path_buf());
    }

    let root = book_root.unwrap_or_else(|| Path::new("."));
    let candidate = root.join("book.toml");
    if candidate.is_file() {
        Ok(candidate)
    } else {
        bail!("Could not find book.toml at {}", candidate.display());
    }
}

pub fn resolve_book_root(book_root: Option<&Path>, config_path: &Path) -> Result<PathBuf> {
    if let Some(book_root) = book_root {
        return Ok(book_root.to_path_buf());
    }

    config_path
        .parent()
        .map(Path::to_path_buf)
        .context("Config path does not have a parent directory")
}

pub fn load_config(path: impl AsRef<Path>) -> Result<Config> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file {}", path.display()))?;
    let parsed: FileConfig = toml::from_str(&contents)
        .with_context(|| format!("Failed to parse TOML in {}", path.display()))?;

    let mut config = Config::default();

    if let Some(book) = parsed.book {
        if let Some(src) = book.src {
            config.book_src = src;
        }
        config.book_title = book.title;
    }

    if let Some(autosummary) = parsed
        .preprocessor
        .and_then(|preprocessor| preprocessor.autosummary)
    {
        if let Some(index_name) = autosummary.index_name {
            config.index_names = vec![index_name];
        }
        if let Some(ignore_hidden) = autosummary.ignore_hidden {
            config.ignore_hidden = ignore_hidden;
        }
    }

    if let Some(native) = parsed.tool.and_then(|tool| tool.mdbook_summary_tools) {
        if let Some(summary_path) = native.summary_path {
            config.summary_path = summary_path;
        }
        if let Some(index_names) = native.index_names {
            config.index_names = index_names;
        }
        if let Some(title_sources) = native.title_source {
            config.title_sources = title_sources;
        }
        if let Some(sort) = native.sort {
            config.sort = sort;
        }
        if let Some(directories_first) = native.directories_first {
            config.directories_first = directories_first;
        }
        if let Some(ignore_hidden) = native.ignore_hidden {
            config.ignore_hidden = ignore_hidden;
        }
        if let Some(exclude) = native.exclude {
            config.exclude = exclude;
        }
        if let Some(include) = native.include {
            config.include = include;
        }
        if let Some(max_depth) = native.max_depth {
            config.max_depth = max_depth;
        }
        if let Some(strip_number_prefixes) = native.strip_number_prefixes {
            config.strip_number_prefixes = strip_number_prefixes;
        }
        if let Some(marker_mode) = native.marker_mode {
            config.marker_mode = marker_mode;
        }
        if let Some(homepage) = native.homepage {
            config.homepage = Some(HomepageConfig {
                title: homepage.title,
                path: homepage.path,
            });
        }
    }

    Ok(config)
}

#[derive(Debug, Deserialize)]
struct FileConfig {
    #[serde(default)]
    book: Option<BookSection>,
    #[serde(default)]
    preprocessor: Option<PreprocessorSection>,
    #[serde(default)]
    tool: Option<ToolSection>,
}

#[derive(Debug, Deserialize)]
struct BookSection {
    #[serde(default)]
    src: Option<String>,
    #[serde(default)]
    title: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PreprocessorSection {
    #[serde(default)]
    autosummary: Option<AutosummaryCompat>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct AutosummaryCompat {
    #[serde(default)]
    index_name: Option<String>,
    #[serde(default)]
    ignore_hidden: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct ToolSection {
    #[serde(rename = "mdbook-summary-tools")]
    #[serde(default)]
    mdbook_summary_tools: Option<NativeConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct NativeConfig {
    #[serde(default)]
    summary_path: Option<String>,
    #[serde(default)]
    index_names: Option<Vec<String>>,
    #[serde(default)]
    title_source: Option<Vec<String>>,
    #[serde(default)]
    sort: Option<String>,
    #[serde(default)]
    directories_first: Option<bool>,
    #[serde(default)]
    ignore_hidden: Option<bool>,
    #[serde(default)]
    exclude: Option<Vec<String>>,
    #[serde(default)]
    include: Option<Vec<String>>,
    #[serde(default)]
    max_depth: Option<usize>,
    #[serde(default)]
    strip_number_prefixes: Option<bool>,
    #[serde(default)]
    marker_mode: Option<String>,
    #[serde(default)]
    homepage: Option<HomepageSection>,
}

#[derive(Debug, Deserialize)]
struct HomepageSection {
    #[serde(default)]
    title: Option<String>,
    path: String,
}
