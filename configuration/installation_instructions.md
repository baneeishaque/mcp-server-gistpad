# GistPad MCP Server

Connects Zed's Agent Panel to your GitHub Gists: browse, create, edit, and comment on gists; optionally add
daily notes, starred/archived gists, and reusable prompts.

## 1. Authenticate (required)

GistPad always uses your own GitHub Personal Access Token. There is no fallback: the extension never uses
the GitHub CLI or any other ambient credentials.

1. Open <https://github.com/settings/tokens/new?scopes=gist>.
2. Name it `Zed GistPad MCP`, select the `gist` scope only, and generate the token.
3. Paste the token into the `github_token` setting below.

Until `github_token` is set, the server refuses to start with an actionable error.

## 2. Optional tool groups

| Setting                 | Flag         | Effect                                 |
| ----------------------- | ------------ | -------------------------------------- |
| `enable_daily_notes`    | `--daily`    | Daily-notes tools and prompts          |
| `enable_starred_gists`  | `--starred`  | Starring tools                         |
| `enable_archived_gists` | `--archived` | Archiving tools                        |
| `enable_prompts`        | `--prompts`  | Reusable prompt management             |
| `markdown_only`         | `--markdown` | Passes the upstream markdown-only flag |

## 3. Requirements

- Zed provides the Node.js runtime: Zed validates Node.js ≥ 22 (using a suitable system Node.js when
  present) and otherwise downloads and manages its own. To guarantee that only Zed's managed Node.js
  is used, set `"node": { "ignore_system_version": true }` in your Zed settings.
- No other system tools are required; the `gistpad-mcp` package is installed by Zed's built-in npm.
