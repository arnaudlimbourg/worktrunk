mod ci_status;
mod layout;
mod model;
mod render;

#[cfg(test)]
mod spacing_test;

use layout::calculate_responsive_layout;
use model::{ListData, ListItem, gather_list_data};
use render::{format_header_line, format_list_item_line};
use worktrunk::git::{GitError, Repository};

pub fn handle_list(
    format: crate::OutputFormat,
    show_branches: bool,
    fetch_ci: bool,
) -> Result<(), GitError> {
    let repo = Repository::current();
    let Some(ListData {
        items,
        current_worktree_path,
    }) = gather_list_data(&repo, show_branches, fetch_ci)?
    else {
        return Ok(());
    };

    match format {
        crate::OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&items).map_err(|e| {
                GitError::CommandFailed(format!("Failed to serialize to JSON: {}", e))
            })?;
            println!("{}", json);
        }
        crate::OutputFormat::Table => {
            let layout = calculate_responsive_layout(&items);
            format_header_line(&layout);
            for item in &items {
                format_list_item_line(item, &layout, current_worktree_path.as_ref());
            }
            display_summary(&items, show_branches);
        }
    }

    Ok(())
}

fn display_summary(items: &[ListItem], include_branches: bool) {
    use anstyle::Style;
    use worktrunk::styling::println;

    if items.is_empty() {
        println!();
        use worktrunk::styling::{HINT, HINT_EMOJI};
        println!("{HINT_EMOJI} {HINT}No worktrees found{HINT:#}");
        println!("{HINT_EMOJI} {HINT}Create one with: wt switch --create <branch>{HINT:#}");
        return;
    }

    let mut metrics = SummaryMetrics::default();
    for item in items {
        metrics.update(item);
    }

    println!();
    let dim = Style::new().dimmed();

    // Build summary parts
    let mut parts = Vec::new();

    if include_branches {
        parts.push(format!("{} worktrees", metrics.worktrees));
        if metrics.branches > 0 {
            parts.push(format!("{} branches", metrics.branches));
        }
    } else {
        let plural = if metrics.worktrees == 1 { "" } else { "s" };
        parts.push(format!("{} worktree{}", metrics.worktrees, plural));
    }

    if metrics.dirty_worktrees > 0 {
        parts.push(format!("{} with changes", metrics.dirty_worktrees));
    }

    if metrics.ahead_items > 0 {
        parts.push(format!("{} ahead", metrics.ahead_items));
    }

    if metrics.behind_items > 0 {
        parts.push(format!("{} behind", metrics.behind_items));
    }

    let summary = parts.join(", ");
    println!("{dim}Showing {summary}{dim:#}");
}

#[derive(Default)]
struct SummaryMetrics {
    worktrees: usize,
    branches: usize,
    dirty_worktrees: usize,
    ahead_items: usize,
    behind_items: usize,
}

impl SummaryMetrics {
    fn update(&mut self, item: &ListItem) {
        if let Some(info) = item.worktree_info() {
            self.worktrees += 1;
            let (added, deleted) = info.working_tree_diff;
            if added > 0 || deleted > 0 {
                self.dirty_worktrees += 1;
            }
        } else {
            self.branches += 1;
        }

        let counts = item.counts();
        if counts.ahead > 0 {
            self.ahead_items += 1;
        }
        if counts.behind > 0 {
            self.behind_items += 1;
        }
    }
}
