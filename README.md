# GistPad MCP Server for Zed

[GistPad](https://github.com/lostintangent/gistpad) for [Zed](https://zed.dev): browse, create, edit, and
comment on GitHub Gists from the Agent Panel, with optional daily notes, starred/archived gists, and
reusable prompts.

Powered by [`gistpad-mcp`](https://github.com/lostintangent/gistpad-mcp).

## Requirements

- Zed 1.22.0 or newer.
- Node.js 22 or newer (Zed's bundled Node is used when compatible; otherwise `node` from `PATH`).
- A GitHub Personal Access Token with the `gist` scope (required; see Configuration).

## Installation

Install from the Zed extension registry (once published), or as a dev extension:

1. In Zed, open the command palette and run "zed: install dev extension".
2. Select this directory.

Zed fetches the pinned `gistpad-mcp` npm package on first use.

## Configuration

Open the Agent Panel, then Settings → AI → MCP Servers → GistPad MCP Server → Configure.

`github_token` is required: the extension has no fallback and never uses the `gh` CLI or any other ambient
credentials. Create a token at <https://github.com/settings/tokens/new?scopes=gist>.

| Setting                 | Default | Description                                       |
| ----------------------- | ------- | ------------------------------------------------- |
| `github_token`          | `""`    | GitHub Personal Access Token with the `gist` scope |
| `enable_daily_notes`    | `false` | Enable daily-notes tools and prompts (`--daily`)  |
| `enable_starred_gists`  | `false` | Enable starring tools (`--starred`)               |
| `enable_archived_gists` | `false` | Enable archiving tools (`--archived`)             |
| `enable_prompts`        | `false` | Enable reusable prompt management (`--prompts`)   |
| `markdown_only`         | `false` | Pass `--markdown` to the server                   |

Example `settings.json`:

```json
{
    "context_servers": {
        "mcp-server-gistpad": {
            "settings": {
                "github_token": "<your-github-token>"
            }
        }
    }
}
```

## Usage

Ask the agent things like:

- "List my 10 most recent gists."
- "Create a gist titled `ideas` with a file `ideas.md`."
- "Add a comment to the `ideas` gist."
- "What are my incomplete todos today?" (requires `enable_daily_notes`)
- "List my saved prompts." (requires `enable_prompts`)

## How it works

The extension installs the pinned `gistpad-mcp` npm package into its work directory and spawns it over
stdio (Node ≥ 22) with `GITHUB_TOKEN` set from the required `github_token` setting. Zed's Agent Panel
talks MCP Tools and Prompts to it.

## Development

```bash
cargo build --target wasm32-wasip2 --release
```

## License

MIT. GistPad and gistpad-mcp are MIT-licensed projects by Jonathan Carter (lostintangent).
