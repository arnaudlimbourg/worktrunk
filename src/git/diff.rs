//! Git diff utilities for parsing and formatting diff statistics.

use crate::styling::{ADDITION, DELETION};

/// Parse git diff --shortstat output
#[derive(Debug)]
pub struct DiffStats {
    pub files: Option<usize>,
    pub insertions: Option<usize>,
    pub deletions: Option<usize>,
}

impl DiffStats {
    /// Format stats as a summary string (e.g., "3 files, +45, -12")
    pub fn format_summary(&self) -> Vec<String> {
        let mut parts = Vec::new();

        if let Some(files) = self.files {
            parts.push(format!(
                "{} file{}",
                files,
                if files == 1 { "" } else { "s" }
            ));
        }
        if let Some(insertions) = self.insertions {
            parts.push(format!("{ADDITION}+{insertions}{ADDITION:#}"));
        }
        if let Some(deletions) = self.deletions {
            parts.push(format!("{DELETION}-{deletions}{DELETION:#}"));
        }

        parts
    }
}

pub fn parse_diff_shortstat(output: &str) -> DiffStats {
    let mut stats = DiffStats {
        files: None,
        insertions: None,
        deletions: None,
    };

    // Example: " 3 files changed, 45 insertions(+), 12 deletions(-)"
    let parts: Vec<&str> = output.split(',').collect();

    for part in parts {
        let part = part.trim();

        if part.contains("file") {
            if let Some(num_str) = part.split_whitespace().next() {
                stats.files = num_str.parse().ok();
            }
        } else if part.contains("insertion") {
            if let Some(num_str) = part.split_whitespace().next() {
                stats.insertions = num_str.parse().ok();
            }
        } else if part.contains("deletion")
            && let Some(num_str) = part.split_whitespace().next()
        {
            stats.deletions = num_str.parse().ok();
        }
    }

    stats
}
