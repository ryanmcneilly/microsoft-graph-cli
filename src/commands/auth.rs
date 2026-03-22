use crate::auth::{device_code, token_store};
use crate::config;

pub fn handle_login() -> Result<(), String> {
    let config = config::load()?;
    let tokens = device_code::run_device_code_flow(&config.client_id, &config.tenant_id)?;
    token_store::save_refresh_token(&tokens.refresh_token)?;
    eprintln!("Authentication successful.");
    Ok(())
}

pub fn handle_logout() -> Result<(), String> {
    token_store::clear_tokens()?;
    eprintln!("Logged out successfully.");
    Ok(())
}
