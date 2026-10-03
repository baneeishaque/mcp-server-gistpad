# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-10-03

### Added

- Zed extension `mcp-server-gistpad`: installs and runs the pinned
  [`gistpad-mcp`](https://github.com/lostintangent/gistpad-mcp) `0.5.0` server for the Agent Panel.
- Settings UI with `github_token`, `use_gh_cli_token` fallback, and optional tool groups
  (`--daily`, `--starred`, `--archived`, `--prompts`, `--markdown`).
- Node.js ≥ 22 runtime gate: Zed's bundled Node first, then `node` on `PATH`, with actionable errors.
- Installation instructions and default settings surfaced in Zed's MCP server configuration.

[0.1.0]: https://github.com/Baneeishaque/mcp-server-gistpad/releases/tag/v0.1.0
