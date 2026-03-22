use serde::Deserialize;
use std::thread;
use std::time::Duration;

const AUTHORITY: &str = "https://login.microsoftonline.com";
const SCOPES: &str = "User.Read Mail.Read offline_access";

pub struct DeviceCodeTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

#[derive(Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
    message: Option<String>,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_in: Option<u64>,
    error: Option<String>,
    error_description: Option<String>,
}

pub fn run_device_code_flow(client_id: &str, tenant_id: &str) -> Result<DeviceCodeTokens, String> {
    let device_code_url = format!("{}/{}/oauth2/v2.0/devicecode", AUTHORITY, tenant_id);
    let body = format!("client_id={}&scope={}", client_id, urlencoded(SCOPES));

    let response: DeviceCodeResponse = {
        let resp = ureq::post(&device_code_url)
            .set("Content-Type", "application/x-www-form-urlencoded")
            .send_string(&body);
        match resp {
            Ok(r) => r
                .into_json()
                .map_err(|e| format!("Failed to parse device code response: {}", e))?,
            Err(ureq::Error::Status(code, r)) => {
                let body = r.into_string().unwrap_or_default();
                return Err(format!(
                    "Device code request failed (HTTP {}): {}",
                    code, body
                ));
            }
            Err(e) => return Err(format!("Failed to request device code: {}", e)),
        }
    };

    // Display instructions to user (stderr to keep stdout clean for AI agents)
    if let Some(msg) = &response.message {
        eprintln!("{}", msg);
    } else {
        eprintln!(
            "To sign in, visit {} and enter code: {}",
            response.verification_uri, response.user_code
        );
    }

    poll_for_token(client_id, tenant_id, &response.device_code, response.interval, response.expires_in)
}

fn poll_for_token(
    client_id: &str,
    tenant_id: &str,
    device_code: &str,
    initial_interval_secs: u64,
    expires_in_secs: u64,
) -> Result<DeviceCodeTokens, String> {
    let token_url = format!("{}/{}/oauth2/v2.0/token", AUTHORITY, tenant_id);
    let mut interval = initial_interval_secs;
    let deadline = std::time::Instant::now() + Duration::from_secs(expires_in_secs);

    loop {
        thread::sleep(Duration::from_secs(interval));

        if std::time::Instant::now() > deadline {
            return Err("Device code expired. Please try again.".to_string());
        }

        let body = format!(
            "grant_type=urn:ietf:params:oauth:grant-type:device_code&client_id={}&device_code={}",
            client_id, device_code
        );

        let resp = ureq::post(&token_url)
            .set("Content-Type", "application/x-www-form-urlencoded")
            .send_string(&body);

        let token: TokenResponse = match resp {
            Ok(r) => r
                .into_json()
                .map_err(|e| format!("Failed to parse token response: {}", e))?,
            Err(ureq::Error::Status(_, r)) => r
                .into_json()
                .map_err(|e| format!("Failed to parse error response: {}", e))?,
            Err(e) => return Err(format!("Network error while polling: {}", e)),
        };

        match token.error.as_deref() {
            None => {
                let access_token = token
                    .access_token
                    .ok_or("Missing access_token in response")?;
                let refresh_token = token
                    .refresh_token
                    .ok_or("Missing refresh_token in response")?;
                let expires_in = token.expires_in.unwrap_or(3600);
                return Ok(DeviceCodeTokens {
                    access_token,
                    refresh_token,
                    expires_in,
                });
            }
            Some("authorization_pending") => {
                // User hasn't signed in yet — keep polling
            }
            Some("slow_down") => {
                // Server asked us to back off
                interval += 5;
            }
            Some("authorization_declined") => {
                return Err("Authorization was declined by the user.".to_string());
            }
            Some("expired_token") => {
                return Err("Device code expired. Please try again.".to_string());
            }
            Some(err) => {
                let desc = token
                    .error_description
                    .unwrap_or_else(|| "No description".to_string());
                return Err(format!("Authentication error: {} — {}", err, desc));
            }
        }
    }
}

/// Percent-encode a string for use in application/x-www-form-urlencoded bodies.
/// Only encodes characters that are not unreserved (RFC 3986) and not safe in form data.
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
