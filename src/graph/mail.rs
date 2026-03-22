use crate::graph::client::GraphClient;
use serde::Serialize;

const MESSAGES_URL: &str = "https://graph.microsoft.com/v1.0/me/messages\
    ?$select=id,subject,from,receivedDateTime,bodyPreview,isRead\
    &$top=20\
    &$orderby=receivedDateTime%20desc";

#[derive(Serialize)]
pub struct MailMessage {
    pub id: String,
    pub subject: String,
    pub from: String,
    pub received_date_time: String,
    pub body_preview: String,
    pub is_read: bool,
}

pub fn list_messages(client: &GraphClient) -> Result<Vec<MailMessage>, String> {
    let json = client.get(MESSAGES_URL)?;

    let values = json["value"]
        .as_array()
        .ok_or("Unexpected response format: missing 'value' array")?;

    values.iter().map(parse_message).collect()
}

fn parse_message(msg: &serde_json::Value) -> Result<MailMessage, String> {
    let id = string_field(msg, "id")?;
    let subject = msg["subject"].as_str().unwrap_or("(no subject)").to_string();
    let from = msg["from"]["emailAddress"]["address"]
        .as_str()
        .unwrap_or("")
        .to_string();
    let received_date_time = string_field(msg, "receivedDateTime")?;
    let body_preview = msg["bodyPreview"].as_str().unwrap_or("").to_string();
    let is_read = msg["isRead"].as_bool().unwrap_or(false);

    Ok(MailMessage {
        id,
        subject,
        from,
        received_date_time,
        body_preview,
        is_read,
    })
}

fn string_field(msg: &serde_json::Value, field: &str) -> Result<String, String> {
    msg[field]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing or invalid field '{}' in message", field))
}
