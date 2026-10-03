# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-10-03

### Added

- Zed extension `mcp-server-gistpad`: installs and runs the pinned
  [`gistpad-mcp`](https://github.com/lostintangent/gistpad-mcp) `0.5.0` server for the Agent Panel.
- Settings UI with a required `github_token` (no fallback to the GitHub CLI or ambient credentials)
  and optional tool groups (`--daily`, `--starred`, `--archived`, `--prompts`, `--markdown`).
- Uses only the Node.js runtime provided by Zed (validated ≥ 22, with Zed's managed Node.js as
  fallback); no system Node or other system tools.
- `zed_extension_api` pinned exactly to `=0.7.0` with committed `Cargo.lock` for reproducible builds.
- Installation instructions and fully commented default settings surfaced in Zed's MCP server
  configuration.

[0.1.0]: https://github.com/baneeishaque/mcp-server-gistpad/releases/tag/v0.1.0
