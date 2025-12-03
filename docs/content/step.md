+++
title = "wt step"
weight = 16

[extra]
group = "Commands"
+++



---

## Command Reference

<!-- ⚠️ AUTO-GENERATED from `wt step --help-page` — edit cli.rs to update -->

```bash
wt step - Workflow building blocks
Usage: wt step [OPTIONS] <COMMAND>

Commands:
  commit       Commit changes with LLM commit message
  squash       Squash commits with LLM commit message
  push         Push changes to local target branch
  rebase       Rebase onto target
  post-create  Run post-create hook
  post-start   Run post-start hook
  pre-commit   Run pre-commit hook
  pre-merge    Run pre-merge hook
  post-merge   Run post-merge hook

Options:
  -h, --help  Print help

Global Options:
  -C <path>            Working directory for this command
      --config <path>  User config file path
  -v, --verbose        Show commands and debug info
```
