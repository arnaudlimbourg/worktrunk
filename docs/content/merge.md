+++
title = "wt merge"
weight = 12

[extra]
group = "Commands"
+++

## Operation

Commit → Squash → Rebase → Pre-merge hooks → Push → Cleanup → Post-merge hooks

### Commit

Uncommitted changes are staged and committed with LLM commit message.
Use `--stage=tracked` to stage only tracked files, or `--stage=none` to commit only what's already staged.

### Squash

Multiple commits are squashed into one (like GitHub's "Squash and merge") with LLM commit message.
Skip with `--no-squash`. Safety backup: `git reflog show refs/wt-backup/<branch>`

### Rebase

Branch is rebased onto target. Conflicts abort the merge immediately.

### Hooks

Pre-merge commands run after rebase (failures abort). Post-merge commands
run after cleanup (failures logged). Skip all with `--no-verify`.

### Push

Fast-forward push to local target branch. Non-fast-forward pushes are rejected.

### Cleanup

Worktree and branch are removed. Skip with `--no-remove`.

**Template variables:** `{{ repo }}`, `{{ branch }}`, `{{ worktree }}`, `{{ repo_root }}`, `{{ target }}`

**Security:** Commands from project hooks require approval on first run.
Approvals are saved to user config. Use `--force` to bypass prompts.
See `wt config approvals --help`.

## Examples

Basic merge to main:

```bash
wt merge
```

Merge without squashing:

```bash
wt merge --no-squash
```

Keep worktree after merging:

```bash
wt merge --no-remove
```

Skip all hooks:

```bash
wt merge --no-verify
```

---

## Command Reference

<!-- ⚠️ AUTO-GENERATED from `wt merge --help-page` — edit cli.rs to update -->

```bash
wt merge - Merge worktree into target branch
Usage: wt merge [OPTIONS] [TARGET]

Arguments:
  [TARGET]
          Target branch

          Defaults to default branch.

Options:
      --no-squash
          Skip commit squashing

      --no-commit
          Skip commit, squash, and rebase

      --no-remove
          Keep worktree after merge

      --no-verify
          Skip all project hooks

  -f, --force
          Skip approval prompts

      --stage <STAGE>
          What to stage before committing [default: all]

          Possible values:
          - all:     Stage everything: untracked files + unstaged tracked changes
          - tracked: Stage tracked changes only (like git add -u)
          - none:    Stage nothing, commit only what's already in the index

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
