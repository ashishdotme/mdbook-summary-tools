# mdbook-summary-tools

Generate `SUMMARY.md` files for `mdBook` projects.

## Install

```bash
cargo install mdbook-summary-tools --version 0.1.0
```

## Commands

```bash
mdbook-summary-tools generate
mdbook-summary-tools generate --stdout
mdbook-summary-tools check --diff
mdbook-summary-tools print-config
```

## Configuration

Example `mdbook-summary-tools.toml`:

```toml
[book]
src = "."
title = "Wiki"

[tool.mdbook-summary-tools]
index-names = ["readme.md", "index.md"]
exclude = ["attachments/**", "book/**"]
ignore-hidden = true

[tool.mdbook-summary-tools.homepage]
title = "wiki.ashish.me"
path = "readme.md"
```

This generates a homepage entry like:

```md
[wiki.ashish.me](<readme.md>)
```

Generate a summary with:

```bash
mdbook-summary-tools generate --book . --config mdbook-summary-tools.toml
```
