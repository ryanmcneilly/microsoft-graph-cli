use std::fs;
use std::path::PathBuf;

pub fn save_refresh_token(refresh_token: &str) -> Result<(), String> {
    let path = token_file_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }
    fs::write(&path, refresh_token)
        .map_err(|e| format!("Failed to save token to {}: {}", path.display(), e))?;

    // Restrict file permissions to owner-only on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))
            .map_err(|e| format!("Failed to set token file permissions: {}", e))?;
    }

    Ok(())
}

pub fn load_refresh_token() -> Result<Option<String>, String> {
    let path = token_file_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let token = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read token from {}: {}", path.display(), e))?;
    let token = token.trim().to_string();
    if token.is_empty() {
        Ok(None)
    } else {
        Ok(Some(token))
    }
}

pub fn clear_tokens() -> Result<(), String> {
    let path = token_file_path()?;
    if path.exists() {
        fs::remove_file(&path)
            .map_err(|e| format!("Failed to remove token file: {}", e))?;
    }
    Ok(())
}

fn token_file_path() -> Result<PathBuf, String> {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map(PathBuf::from)
        .map_err(|_| "Cannot determine home directory (USERPROFILE/HOME not set)".to_string())?;
    Ok(home.join(".config").join("msgraph").join("tokens"))
}
