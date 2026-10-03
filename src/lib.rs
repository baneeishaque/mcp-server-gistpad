use schemars::JsonSchema;
use serde::Deserialize;
use zed_extension_api::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "gistpad-mcp";
const PACKAGE_VERSION: &str = "0.5.0";
const SERVER_PATH: &str = "node_modules/gistpad-mcp/build/index.js";
const CONTEXT_SERVER_ID: &str = "mcp-server-gistpad";
const MIN_NODE_MAJOR: u32 = 22;

#[derive(Debug, Default, Deserialize, JsonSchema)]
struct GistpadContextServerSettings {
    /// GitHub Personal Access Token with only the `gist` scope. Required: the
    /// extension never falls back to `gh auth` or any other ambient credentials.
    github_token: Option<String>,
    /// Expose daily-notes tools and prompts (default: false).
    #[serde(default)]
    enable_daily_notes: bool,
    /// Expose starring tools (default: false).
    #[serde(default)]
    enable_starred_gists: bool,
    /// Expose archiving tools (default: false).
    #[serde(default)]
    enable_archived_gists: bool,
    /// Expose reusable-prompt tools and prompts (default: false).
    #[serde(default)]
    enable_prompts: bool,
    /// Pass `--markdown` to the server (default: false).
    #[serde(default)]
    markdown_only: bool,
}

struct GistpadExtension;

impl GistpadExtension {
    fn ensure_server_installed() -> Result<()> {
        let installed = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if installed.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }
        Ok(())
    }

    fn resolve_token(settings: &GistpadContextServerSettings) -> Result<String> {
        if let Some(token) = settings
            .github_token
            .as_deref()
            .map(str::trim)
            .filter(|token| !token.is_empty())
        {
            return Ok(token.to_string());
        }

        Err("A GitHub Personal Access Token with the `gist` scope is required. Set `github_token` \
             in the GistPad MCP server settings; create one at \
             https://github.com/settings/tokens/new?scopes=gist."
            .to_string())
    }

    fn node_supports_server() -> Result<String> {
        let bundled = zed::node_binary_path()?;
        if node_major(&bundled).is_some_and(|major| major >= MIN_NODE_MAJOR) {
            return Ok(bundled);
        }

        if let Ok(output) = zed::process::Command::new("node").arg("--version").output() {
            if output.status == Some(0) {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if node_major_from_version(&version).is_some_and(|major| major >= MIN_NODE_MAJOR) {
                    return Ok("node".to_string());
                }
            }
        }

        Err(format!(
            "GistPad MCP requires Node >= {MIN_NODE_MAJOR}. Zed's bundled Node ({bundled}) is too old \
             and no suitable `node` was found on PATH. Install Node 22+ (e.g. `mise use -g node@22`)."
        ))
    }

    fn server_args(settings: &GistpadContextServerSettings) -> Result<Vec<String>> {
        let mut args = vec![std::env::current_dir()
            .map_err(|err| err.to_string())?
            .join(SERVER_PATH)
            .to_string_lossy()
            .to_string()];
        if settings.enable_daily_notes {
            args.push("--daily".to_string());
        }
        if settings.enable_starred_gists {
            args.push("--starred".to_string());
        }
        if settings.enable_archived_gists {
            args.push("--archived".to_string());
        }
        if settings.enable_prompts {
            args.push("--prompts".to_string());
        }
        if settings.markdown_only {
            args.push("--markdown".to_string());
        }
        Ok(args)
    }
}

fn node_major(path: &str) -> Option<u32> {
    let output = zed::process::Command::new(path)
        .arg("--version")
        .output()
        .ok()?;
    if output.status != Some(0) {
        return None;
    }
    node_major_from_version(String::from_utf8_lossy(&output.stdout).trim())
}

fn node_major_from_version(version: &str) -> Option<u32> {
    version.trim_start_matches('v').split('.').next()?.parse().ok()
}

impl zed::Extension for GistpadExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        Self::ensure_server_installed()?;

        let settings = ContextServerSettings::for_project(CONTEXT_SERVER_ID, project)?;
        let settings: GistpadContextServerSettings = match settings.settings {
            Some(value) => serde_json::from_value(value).map_err(|err| err.to_string())?,
            None => GistpadContextServerSettings::default(),
        };

        let token = Self::resolve_token(&settings)?;

        Ok(Command {
            command: Self::node_supports_server()?,
            args: Self::server_args(&settings)?,
            env: vec![("GITHUB_TOKEN".to_string(), token)],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(GistpadContextServerSettings))
                .map_err(|err| err.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(GistpadExtension);
