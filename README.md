# mdbook-generate-summary

Generate `SUMMARY.md` files for `mdBook` projects.


## Commands

```bash
mdbook-generate-summary generate
mdbook-generate-summary generate --stdout
mdbook-generate-summary check --diff
mdbook-generate-summary print-config
```

## Configuration

Native configuration lives in `book.toml` under `[tool.mdbook-generate-summary]`.

```toml
[tool.mdbook-generate-summary]
index-names = ["readme.md", "index.md"]
exclude = ["attachments/**", "book/**"]
ignore-hidden = true

[tool.mdbook-generate-summary.homepage]
title = "wiki.ashish.me"
path = "readme.md"
```

This generates a homepage entry like:

```md
[wiki.ashish.me](<readme.md>)
```
