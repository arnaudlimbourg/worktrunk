+++
title = "wt config"
weight = 15

[extra]
group = "Commands"
+++

## Setup Guide

1. Set up shell integration

   ```bash
   wt config shell install
   ```

   Or manually add to the shell config:

   ```bash
   eval "$(wt config shell init bash)"
   ```

2. (Optional) Create user config file

   ```bash
   wt config create
   ```

   This creates `~/.config/worktrunk/config.toml` with examples.

3. (Optional) Enable LLM commit messages

   Install: `uv tool install -U llm`
   Configure: `llm keys set anthropic`
   Add to user config:

   ```toml
   [commit-generation]
   command = "llm"
   ```

## LLM Setup Details

For Claude:

```bash
llm install llm-anthropic
llm keys set anthropic
llm models default claude-haiku-4-5-20251001
```

For OpenAI:

```bash
llm keys set openai
```

Use `wt config show` to view the current configuration.
Docs: <https://llm.datasette.io/> | <https://github.com/sigoden/aichat>

## Configuration Files

**User config**:

- Location: `~/.config/worktrunk/config.toml` (or `WORKTRUNK_CONFIG_PATH`)
- Run `wt config create --help` to view documented examples

**Project config**:

- Location: `.config/wt.toml` in repository root
- Contains: post-create, post-start, pre-commit, pre-merge, post-merge hooks

---

## Command Reference

<!-- ⚠️ AUTO-GENERATED from `wt config --help-page` — edit cli.rs to update -->

```bash
wt config - Manage configuration and shell integration
Usage: wt config [OPTIONS] <COMMAND>

Commands:
  shell      Shell integration setup
  create     Create user configuration file
  show       Show configuration files & locations
  cache      Manage caches (CI status, default branch)
  var        Get or set runtime variables (stored in git config)
  approvals  Manage command approvals

Options:
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
