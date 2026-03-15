# mdbook-summary-tools

Generate `SUMMARY.md` files for `mdBook` projects.


## Commands

```bash
mdbook-summary-tools generate
mdbook-summary-tools generate --stdout
mdbook-summary-tools check --diff
mdbook-summary-tools print-config
```

## Configuration

Native configuration lives in `book.toml` under `[tool.mdbook-summary-tools]`.

```toml
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
