# GistPad MCP Server

Connects Zed's Agent Panel to your GitHub Gists: browse, create, edit, and comment on gists; optionally add
daily notes, starred/archived gists, and reusable prompts.

## 1. Authenticate

Option A — Personal Access Token (recommended):

1. Open <https://github.com/settings/tokens/new?scopes=gist>.
2. Name it `Zed GistPad MCP`, select the `gist` scope only, and generate the token.
3. Paste the token into the `github_token` setting below.

Option B — GitHub CLI fallback (leave `github_token` empty and keep `use_gh_cli_token` enabled):

```bash
gh auth login          # ensure the `gist` scope is granted
gh auth refresh -s gist
```

## 2. Optional tool groups

| Setting                 | Flag         | Effect                                 |
| ----------------------- | ------------ | -------------------------------------- |
| `enable_daily_notes`    | `--daily`    | Daily-notes tools and prompts          |
| `enable_starred_gists`  | `--starred`  | Starring tools                         |
| `enable_archived_gists` | `--archived` | Archiving tools                        |
| `enable_prompts`        | `--prompts`  | Reusable prompt management             |
| `markdown_only`         | `--markdown` | Passes the upstream markdown-only flag |

## 3. Requirements

- Node.js 22 or newer. Zed's bundled Node is used when it is 22+; otherwise a `node` on `PATH` is used.
- The extension uses the `npm:install` and `process:exec` capabilities (Zed prompts on first use).
