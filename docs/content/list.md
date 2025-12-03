+++
title = "wt list"
weight = 11

[extra]
group = "Commands"
+++

## Columns

- **Branch:** Branch name
- **Status:** Quick status symbols (see Status Symbols below)
- **HEAD±:** Uncommitted changes vs HEAD (+added -deleted lines, staged + unstaged)
- **main↕:** Commit count ahead↑/behind↓ relative to main (commits in HEAD vs main)
- **main…±** (`--full`): Line diffs in commits ahead of main (+added -deleted)
- **Path:** Worktree directory location
- **Remote⇅:** Commits ahead⇡/behind⇣ relative to tracking branch (e.g. `origin/branch`)
- **CI** (`--full`): CI pipeline status (tries PR/MR checks first, falls back to branch workflows)
  - `●` **passed** (green) - All checks passed
  - `●` **running** (blue) - Checks in progress
  - `●` **failed** (red) - Checks failed
  - `●` **conflicts** (yellow) - Merge conflicts with base
  - `●` **no-ci** (gray) - PR/MR or workflow found but no checks configured
  - **(blank)** - No PR/MR or workflow found, or `gh`/`glab` CLI unavailable
  - **(dimmed)** - Stale: unpushed local changes differ from PR/MR head
- **Commit:** Short commit hash (8 chars)
- **Age:** Time since last commit (relative)
- **Message:** Last commit message (truncated)

## Status Symbols

Order: `+!? ✖⚠≡_ ↻⋈ ↑↓↕ ⇡⇣⇅ ⎇⌫⊠`

- `+` Staged files (ready to commit)
- `!` Modified files (unstaged changes)
- `?` Untracked files present
- `✖` **Merge conflicts** - unresolved conflicts in working tree (fix before continuing)
- `⊘` **Would conflict** - merging into main would fail
- `≡` Working tree matches main (identical contents, regardless of commit history)
- `_` No commits (no commits ahead AND no uncommitted changes)
- `↻` Rebase in progress
- `⋈` Merge in progress
- `↑` Ahead of main branch
- `↓` Behind main branch
- `↕` Diverged (both ahead and behind main)
- `⇡` Ahead of remote tracking branch
- `⇣` Behind remote tracking branch
- `⇅` Diverged (both ahead and behind remote)
- `⎇` Branch indicator (shown for branches without worktrees)
- `⌫` Prunable worktree (directory missing, can be pruned)
- `⊠` Locked worktree (protected from auto-removal)

Rows are dimmed when there's no marginal contribution (`≡` matches main OR `_` no commits).

## JSON Output

Use `--format=json` for structured data. Each object contains two status maps
with the same fields in the same order as Status Symbols above:

**`status`** - variant names for querying:

- `working_tree`: `{untracked, modified, staged, renamed, deleted}` booleans
- `branch_state`: `""` | `"Conflicts"` | `"MergeTreeConflicts"` | `"MatchesMain"` | `"NoCommits"`
- `git_operation`: `""` | `"Rebase"` | `"Merge"`
- `main_divergence`: `""` | `"Ahead"` | `"Behind"` | `"Diverged"`
- `upstream_divergence`: `""` | `"Ahead"` | `"Behind"` | `"Diverged"`
- `user_marker`: string (optional)

**`status_symbols`** - Unicode symbols for display (same fields, plus `worktree_attrs`: ⎇/⌫/⊠)

Note: `locked` and `prunable` are top-level fields on worktree objects, not in status.

**Worktree position fields** (for identifying special worktrees):

- `is_main`: boolean - is the main worktree
- `is_current`: boolean - is the current working directory (present when true)
- `is_previous`: boolean - is the previous worktree from `wt switch` (present when true)

**Query examples:**

```bash
# Find worktrees with conflicts
jq '.[] | select(.status.branch_state == "Conflicts")'

# Find worktrees with untracked files
jq '.[] | select(.status.working_tree.untracked)'

# Find worktrees in rebase or merge
jq '.[] | select(.status.git_operation != "")'

# Get branches ahead of main
jq '.[] | select(.status.main_divergence == "Ahead")'

# Find locked worktrees
jq '.[] | select(.locked != null)'

# Get current worktree info (useful for statusline tools)
jq '.[] | select(.is_current == true)'
```

---

## Command Reference

<!-- ⚠️ AUTO-GENERATED from `wt list --help-page` — edit cli.rs to update -->

```bash
wt list - List worktrees and optionally branches
Usage: wt list [OPTIONS]
       wt list <COMMAND>

Commands:
  statusline  Single-line status for shell prompts

Options:
      --format <FORMAT>
          Output format (table, json)

          [default: table]

      --branches
          Include branches without worktrees

      --remotes
          Include remote branches

      --full
          Show CI, conflicts, diffs

      --progressive
          Show fast info immediately, update with slow info

          Displays local data (branches, paths, status) first, then updates with remote data (CI, upstream) as it arrives. Auto-enabled for TTY.

  -h, --help
          Print help (see a summary with '-h')

Global Options:
  -C <path>
          Working directory for this command

      --config <path>
          User config file path

  -v, --verbose
          Show commands and debug info
```
