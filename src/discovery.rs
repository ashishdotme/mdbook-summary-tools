use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};

use crate::config::Config;
use crate::model::{BookTree, Node, Page, Section};
use crate::titles::resolve_title_with_sources;

pub fn discover_book(root: impl AsRef<Path>, config: &Config) -> Result<BookTree> {
    let root = root.as_ref();
    let source_root = root.join(&config.book_src);
    let filters = Filters::new(&config.exclude, &config.include)?;
    let homepage = config.homepage.as_ref().map(|homepage| {
        let homepage_path = source_root.join(&homepage.path);
        let title = homepage.title.clone().unwrap_or_else(|| {
            resolve_title_with_sources(&homepage_path, &config.title_sources)
                .unwrap_or_else(|_| title_from_path(Path::new(&homepage.path)))
        });
        Page {
            title,
            path: normalize_path(Path::new(&homepage.path)),
        }
    });

    let root_items = collect_dir(
        &source_root,
        &source_root,
        config,
        &filters,
        homepage.as_ref(),
        true,
    )?;
    let mut paths = BTreeSet::new();
    if let Some(homepage) = &homepage {
        paths.insert(homepage.path.clone());
    }
    collect_paths(&root_items, &mut paths);

    Ok(BookTree {
        homepage,
        root_items,
        paths,
    })
}

fn collect_dir(
    dir: &Path,
    source_root: &Path,
    config: &Config,
    filters: &Filters,
    homepage: Option<&Page>,
    is_root: bool,
) -> Result<Vec<Node>> {
    let mut entries = fs::read_dir(dir)
        .with_context(|| format!("Failed to read directory {}", dir.display()))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .with_context(|| format!("Failed to list directory {}", dir.display()))?;
    entries.sort_by_key(|entry| entry.file_name());

    let mut nodes = Vec::new();

    for entry in entries {
        let path = entry.path();
        let file_type = entry
            .file_type()
            .with_context(|| format!("Failed to inspect {}", path.display()))?;
        let relative_path = path.strip_prefix(source_root).unwrap_or(path.as_path());

        if is_hidden(&path, config.ignore_hidden) {
            continue;
        }

        if file_type.is_dir() {
            if let Some(section) =
                discover_section(&path, relative_path, source_root, config, filters, homepage)?
            {
                nodes.push(Node::Section(section));
            }
            continue;
        }

        if !file_type.is_file() || !is_markdown(&path) {
            continue;
        }

        let relative = normalize_path(relative_path);
        if relative == "SUMMARY.md"
            || filters.is_excluded(relative_path)
            || !filters.is_included(relative_path)
        {
            continue;
        }
        if homepage.is_some_and(|page| page.path == relative) && is_root {
            continue;
        }
        if !is_root
            && path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| config.index_names.iter().any(|index| index == name))
        {
            continue;
        }

        nodes.push(Node::Page(Page {
            title: resolve_title_with_sources(&path, &config.title_sources)
                .unwrap_or_else(|_| title_from_path(relative_path)),
            path: relative,
        }));
    }

    Ok(nodes)
}

fn discover_section(
    path: &Path,
    relative_path: &Path,
    source_root: &Path,
    config: &Config,
    filters: &Filters,
    homepage: Option<&Page>,
) -> Result<Option<Section>> {
    let Some(index_path) = find_index_file(path, config) else {
        return Ok(None);
    };

    let relative_index = index_path
        .strip_prefix(source_root)
        .unwrap_or(index_path.as_path());
    if filters.is_excluded(relative_index) || !filters.is_included(relative_index) {
        return Ok(None);
    }
    if homepage.is_some_and(|page| page.path == normalize_path(relative_index)) {
        return Ok(None);
    }

    let children = collect_dir(path, source_root, config, filters, homepage, false)?;
    Ok(Some(Section {
        title: resolve_title_with_sources(&index_path, &config.title_sources)
            .unwrap_or_else(|_| title_from_path(relative_path)),
        path: normalize_path(relative_index),
        children,
    }))
}

fn find_index_file(dir: &Path, config: &Config) -> Option<PathBuf> {
    config
        .index_names
        .iter()
        .map(|index_name| dir.join(index_name))
        .find(|candidate| candidate.is_file())
}

fn collect_paths(nodes: &[Node], paths: &mut BTreeSet<String>) {
    for node in nodes {
        match node {
            Node::Page(page) => {
                paths.insert(page.path.clone());
            }
            Node::Section(section) => {
                paths.insert(section.path.clone());
                collect_paths(&section.children, paths);
            }
        }
    }
}

fn is_markdown(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
}

fn is_hidden(path: &Path, ignore_hidden: bool) -> bool {
    ignore_hidden
        && path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with('.') || name.starts_with('_'))
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn title_from_path(path: &Path) -> String {
    let raw = path
        .file_stem()
        .or_else(|| path.file_name())
        .and_then(|segment| segment.to_str())
        .unwrap_or_default();

    if matches!(raw, "readme" | "README" | "index" | "INDEX") {
        path.parent()
            .and_then(|parent| parent.file_name())
            .and_then(|segment| segment.to_str())
            .map(title_case)
            .unwrap_or_else(|| title_case(raw))
    } else {
        title_case(raw)
    }
}

fn title_case(raw: &str) -> String {
    raw.split(['-', '_', ' '])
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => {
                    let mut word = String::new();
                    word.extend(first.to_uppercase());
                    word.push_str(chars.as_str());
                    word
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

struct Filters {
    exclude: GlobSet,
    include: Option<GlobSet>,
}

impl Filters {
    fn new(exclude: &[String], include: &[String]) -> Result<Self> {
        Ok(Self {
            exclude: build_globset(exclude)?,
            include: if include.is_empty() {
                None
            } else {
                Some(build_globset(include)?)
            },
        })
    }

    fn is_excluded(&self, path: &Path) -> bool {
        self.exclude.is_match(path)
    }

    fn is_included(&self, path: &Path) -> bool {
        self.include
            .as_ref()
            .is_none_or(|include| include.is_match(path))
    }
}

fn build_globset(patterns: &[String]) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        builder.add(Glob::new(pattern)?);
    }
    builder.build().context("Failed to build glob matcher")
}
