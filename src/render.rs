use anyhow::Result;

use crate::model::{BookTree, Node, Page, Section};
use crate::validate::validate_summary;

pub fn render_summary(tree: &BookTree) -> String {
    let mut lines = Vec::new();

    if let Some(homepage) = &tree.homepage {
        lines.push(render_page(homepage, 0));
    }

    for node in &tree.root_items {
        render_node(node, 0, &mut lines);
    }

    if lines.is_empty() {
        String::new()
    } else {
        format!("{}\n", lines.join("\n"))
    }
}

pub fn render_validated_summary(tree: &BookTree) -> Result<String> {
    let summary = render_summary(tree);
    validate_summary(&summary)?;
    Ok(summary)
}

fn render_node(node: &Node, depth: usize, lines: &mut Vec<String>) {
    match node {
        Node::Page(page) => lines.push(render_page(page, depth)),
        Node::Section(section) => render_section(section, depth, lines),
    }
}

fn render_page(page: &Page, depth: usize) -> String {
    if depth == 0 {
        format!("[{}](<{}>)", page.title, page.path)
    } else {
        format!("{}- [{}](<{}>)", indent(depth), page.title, page.path)
    }
}

fn render_section(section: &Section, depth: usize, lines: &mut Vec<String>) {
    lines.push(format!(
        "{}- [{}](<{}>)",
        indent(depth),
        section.title,
        section.path
    ));

    for child in &section.children {
        render_node(child, depth + 1, lines);
    }
}

fn indent(depth: usize) -> String {
    "  ".repeat(depth)
}
