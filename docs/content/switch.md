+++
title = "wt switch"
weight = 10

[extra]
group = "Commands"
+++

## Operation

### Worktree resolution

Arguments are resolved using **path-first lookup**:

1. Compute the expected path for the argument (using the configured path template)
2. If a worktree exists at that path, switch to it (regardless of what branch it's on)
3. Otherwise, treat the argument as a branch name

**Example**: If `repo.foo/` exists but is on branch `bar`:

- `wt switch foo` switches to `repo.foo/` (the `bar` branch worktree)
- `wt switch bar` also works (falls back to branch lookup)

### Switching to Existing Worktree

- If worktree exists at expected path or for branch, changes directory via shell integration
- No hooks run
- No branch creation

### Creating New Worktree (`--create`)

1. Creates new branch (defaults to current default branch as base)
2. Creates worktree in configured location (default: `../{{ main_worktree }}.{{ branch }}`)
3. Runs post-create hooks sequentially (blocking)
4. Shows success message
5. Spawns post-start hooks in background (non-blocking)
6. Changes directory to new worktree via shell integration

## Hooks

### post-create (sequential, blocking)

- Run after worktree creation, before success message
- Typically: `npm install`, `cargo build`, setup tasks
- Failures block the operation
- Skip with `--no-verify`

### post-start (parallel, background)

- Spawned after success message shown
- Typically: dev servers, file watchers, editors
- Run in background, failures logged but don't block
- Logs: `.git/wt-logs/{branch}-post-start-{name}.log`
- Skip with `--no-verify`

**Template variables:** `{{ repo }}`, `{{ branch }}`, `{{ worktree }}`, `{{ repo_root }}`

**Security:** Commands from project hooks require approval on first run.
Approvals are saved to user config. Use `--force` to bypass prompts.
See `wt config approvals --help`.

## Examples

Switch to existing worktree:

```bash
wt switch feature-branch
```

Create new worktree from main:

```bash
wt switch --create new-feature
```

Switch to previous worktree:

```bash
wt switch -
```

Create from specific base:

```bash
wt switch --create hotfix --base production
```

Create and run command:

```bash
wt switch --create docs --execute "code ."
```

Skip hooks during creation:

```bash
wt switch --create temp --no-verify
```

## Shortcuts

Use `@` for current HEAD, `-` for previous, `^` for main:

```bash
wt switch @                              # Switch to current branch's worktree
wt switch -                              # Switch to previous worktree
wt switch --create new-feature --base=^  # Branch from main (default)
wt switch --create bugfix --base=@       # Branch from current HEAD
wt remove @                              # Remove current worktree
```

---

## Command Reference

<!-- ⚠️ AUTO-GENERATED from `wt switch --help-page` — edit cli.rs to update -->

```bash
wt switch - Switch to a worktree
Usage: wt switch [OPTIONS] <BRANCH>

Arguments:
  <BRANCH>
          Branch or worktree name

          Shortcuts: '^' (main), '-' (previous), '@' (current)

Options:
  -c, --create
          Create a new branch

  -b, --base <BASE>
          Base branch

          Defaults to default branch.

  -x, --execute <EXECUTE>
          Command to run after switch

  -f, --force
          Skip approval prompts

      --no-verify
          Skip all project hooks

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
