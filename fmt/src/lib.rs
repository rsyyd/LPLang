//! LPLang Code Formatter

pub mod formatter;

use anyhow::Result;
use std::path::Path;

/// Format a file or directory.
pub fn format_path(path: &Path, check_only: bool) -> Result<()> {
    if path.is_file() {
        format_file(path, check_only)
    } else {
        for entry in walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "lp"))
        {
            format_file(entry.path(), check_only)?;
        }
        Ok(())
    }
}

fn format_file(path: &Path, check_only: bool) -> Result<()> {
    let source = std::fs::read_to_string(path)?;
    let formatted = formatter::format(&source)?;

    if check_only {
        if source != formatted {
            anyhow::bail!("{} needs formatting", path.display());
        }
    } else if source != formatted {
        std::fs::write(path, formatted)?;
    }
    Ok(())
}