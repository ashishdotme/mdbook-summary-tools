use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page {
    pub title: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub title: String,
    pub path: String,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Page(Page),
    Section(Section),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BookTree {
    pub homepage: Option<Page>,
    pub root_items: Vec<Node>,
    pub paths: BTreeSet<String>,
}

impl BookTree {
    pub fn root_markdown_occurrences(&self, path: &str) -> usize {
        self.root_items
            .iter()
            .filter(|node| match node {
                Node::Page(page) => page.path == path,
                Node::Section(section) => section.path == path,
            })
            .count()
    }
}
