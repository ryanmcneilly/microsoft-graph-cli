use std::fs;
use std::path::PathBuf;

pub struct Config {
    pub client_id: String,
    /// Tenant ID used in OAuth2 URLs. Defaults to "common" (multi-tenant + personal accounts).
    /// Set to your directory tenant ID if your app registration is single-tenant.
    pub tenant_id: String,
}

pub fn load() -> Result<Config, String> {
    let config_path = config_file_path()?;
    let file_contents = if config_path.exists() {
        Some(
            fs::read_to_string(&config_path)
                .map_err(|e| format!("Failed to read config file {}: {}", config_path.display(), e))?,
        )
    } else {
        None
    };

    let client_id = std::env::var("MSGRAPH_CLIENT_ID")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| file_contents.as_deref().and_then(|c| parse_value(c, "auth", "client_id")))
        .ok_or_else(|| format!(
            "No client ID found. Set MSGRAPH_CLIENT_ID env var or add to {}:\n\n[auth]\nclient_id = \"<your-azure-app-client-id>\"",
            config_path.display()
        ))?;

    let tenant_id = std::env::var("MSGRAPH_TENANT_ID")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| file_contents.as_deref().and_then(|c| parse_value(c, "auth", "tenant_id")))
        .unwrap_or_else(|| "common".to_string());

    Ok(Config { client_id, tenant_id })
}

fn config_file_path() -> Result<PathBuf, String> {
    let home = dirs_home()?;
    Ok(home.join(".config").join("msgraph").join("config.toml"))
}

/// Parse a string value from a minimal TOML file without a toml crate.
/// Looks for `key = "..."` under the given `[section]`.
fn parse_value(contents: &str, section: &str, key: &str) -> Option<String> {
    let section_header = format!("[{}]", section);
    let mut in_section = false;
    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_section = trimmed == section_header;
            continue;
        }
        if in_section {
            if let Some(value) = extract_string_value(trimmed, key) {
                return Some(value);
            }
        }
    }
    None
}

/// Extract the string value from a line like `key = "value"`.
fn extract_string_value(line: &str, key: &str) -> Option<String> {
    let prefix = format!("{} =", key);
    let line = line.trim();
    if !line.starts_with(&prefix) {
        return None;
    }
    let rest = line[prefix.len()..].trim();
    if rest.starts_with('"') && rest.ends_with('"') && rest.len() >= 2 {
        Some(rest[1..rest.len() - 1].to_string())
    } else {
        None
    }
}

fn dirs_home() -> Result<PathBuf, String> {
    std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map(PathBuf::from)
        .map_err(|_| "Cannot determine home directory (USERPROFILE/HOME not set)".to_string())
}
