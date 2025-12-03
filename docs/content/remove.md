+++
title = "wt remove"
weight = 13

[extra]
group = "Commands"
+++

## Operation

Removes worktree directory, git metadata, and branch. Requires clean working tree.

### No arguments (remove current)

- Removes current worktree and switches to main worktree
- In main worktree: switches to default branch

### By name (remove specific)

- Removes specified worktree(s) and branches
- Current worktree removed last (switches to main first)

### Worktree resolution

Arguments are resolved to worktrees using **path-first lookup**:

1. Compute the expected path for the argument (using the configured path template)
2. If a worktree exists at that path, use it (regardless of what branch it's on)
3. Otherwise, treat the argument as a branch name

**Example**: If `repo.foo/` exists but is on branch `bar`:

- `wt remove foo` removes `repo.foo/` and the `bar` branch
- `wt remove bar` also works (falls back to branch lookup)

**Conflict detection**: If path `repo.foo/` has a worktree on branch `bar`, but
branch `foo` has a different worktree at `repo.bar/`, an error is raised.

**Special arguments**:

- `@` - current worktree (by path, works in detached HEAD)
- `-` - previous worktree (from switch history)
- `^` - main worktree

### Branch deletion

By default, branches are deleted only when their content is already in the target branch:

- no changes beyond the common ancestor — `git diff --name-only target...branch` is empty:
  no files changed between the merge base of `target`/`branch` and the tip of `branch`.
- same content as target — `git rev-parse branch^{tree}` equals `git rev-parse target^{tree}`:
  both branches point at the same tracked-files snapshot (tree), even if the commits differ.

This handles workflows where PRs are squash-merged or rebased, which don't preserve
commit ancestry but do integrate the content. Use `-D` to delete unintegrated
branches, or `--no-delete-branch` to always keep branches.

### Background removal (default)

- Returns immediately for continued work
- Logs: `.git/wt-logs/{branch}-remove.log`
- Use `--no-background` for foreground (blocking)

### Cleanup

Stops any git fsmonitor daemon for the worktree before removal. This prevents orphaned processes when using builtin fsmonitor (`core.fsmonitor=true`). No effect on Watchman users.

## Examples

Remove current worktree and branch:

```bash
wt remove
```

Remove specific worktree and branch:

```bash
wt remove feature-branch
```

Remove worktree but keep branch:

```bash
wt remove --no-delete-branch feature-branch
```

Remove multiple worktrees:

```bash
wt remove old-feature another-branch
```

Remove in foreground (blocking):

```bash
wt remove --no-background feature-branch
```

Switch to default in main:

```bash
wt remove  # (when already in main worktree)
```

---

## Command Reference

<!-- ⚠️ AUTO-GENERATED from `wt remove --help-page` — edit cli.rs to update -->

```bash
wt remove - Remove worktree and branch
Usage: wt remove [OPTIONS] [WORKTREES]...

Arguments:
  [WORKTREES]...
          Worktree or branch (@ for current)

Options:
      --no-delete-branch
          Keep branch after removal

  -D, --force-delete
          Delete unmerged branches

      --no-background
          Run removal in foreground

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
