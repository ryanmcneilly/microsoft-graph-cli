use crate::config;
use crate::graph::{client::GraphClient, mail};

pub fn handle_mail_list() -> Result<(), String> {
    let config = config::load()?;
    let client = GraphClient::new(config.client_id, &config.tenant_id)?;
    let messages = mail::list_messages(&client)?;
    let json = serde_json::to_string_pretty(&messages)
        .map_err(|e| format!("Failed to serialize messages: {}", e))?;
    println!("{}", json);
    Ok(())
}
