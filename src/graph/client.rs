use crate::auth::token_store;
use serde::Deserialize;

const AUTHORITY: &str = "https://login.microsoftonline.com";
const SCOPES: &str = "User.Read Mail.Read offline_access";

pub struct GraphClient {
    pub client_id: String,
    pub access_token: String,
}

impl GraphClient {
    /// Create a new GraphClient by exchanging the stored refresh token for a fresh access token.
    pub fn new(client_id: String, tenant_id: &str) -> Result<Self, String> {
        let refresh_token = token_store::load_refresh_token()?.ok_or(
            "Not authenticated. Run `msgraph auth login` first.".to_string(),
        )?;

        let token_url = format!("{}/{}/oauth2/v2.0/token", AUTHORITY, tenant_id);
        let refreshed = refresh_access_token(&client_id, &refresh_token, &token_url)?;

        // Persist the new refresh token (Microsoft may rotate it)
        token_store::save_refresh_token(&refreshed.refresh_token)?;

        Ok(Self {
            client_id,
            access_token: refreshed.access_token,
        })
    }

    /// Perform a GET request to the Graph API and return the parsed JSON value.
    pub fn get(&self, url: &str) -> Result<serde_json::Value, String> {
        let response = ureq::get(url)
            .set("Authorization", &format!("Bearer {}", self.access_token))
            .set("Accept", "application/json")
            .call()
            .map_err(|e| format!("Graph API request failed: {}", e))?;

        response
            .into_json::<serde_json::Value>()
            .map_err(|e| format!("Failed to parse Graph API response: {}", e))
    }
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    refresh_token: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

struct RefreshedTokens {
    access_token: String,
    refresh_token: String,
}

fn refresh_access_token(client_id: &str, refresh_token: &str, token_url: &str) -> Result<RefreshedTokens, String> {
    let body = format!(
        "grant_type=refresh_token&client_id={}&refresh_token={}&scope={}",
        client_id,
        refresh_token,
        urlencoded(SCOPES)
    );

    let resp = ureq::post(token_url)
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_string(&body)
        .map_err(|e| format!("Token refresh request failed: {}", e))?;

    let token: TokenResponse = resp
        .into_json()
        .map_err(|e| format!("Failed to parse refresh response: {}", e))?;

    if let Some(err) = token.error {
        let desc = token.error_description.unwrap_or_default();
        return Err(format!(
            "Token refresh failed: {} — {}. Please run `msgraph auth login` again.",
            err, desc
        ));
    }

    Ok(RefreshedTokens {
        access_token: token.access_token.ok_or("Missing access_token in refresh response")?,
        refresh_token: token.refresh_token.ok_or("Missing refresh_token in refresh response")?,
    })
}

fn urlencoded(s: &str) -> String {
    s.chars()
        .flat_map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '~') {
                vec![c]
            } else if c == ' ' {
                vec!['+']
            } else {
                let byte = c as u8;
                format!("%{:02X}", byte).chars().collect()
            }
        })
        .collect()
}
