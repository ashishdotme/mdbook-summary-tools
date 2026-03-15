use anyhow::Result;
use mdbook_summary::parse_summary;

pub fn validate_summary(summary: &str) -> Result<()> {
    parse_summary(summary)?;
    Ok(())
}
