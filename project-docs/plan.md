# msgraph CLI — Phase 1 Proof of Concept

## Overview

`msgraph` is a token-efficient CLI tool for AI agents to interact with the Microsoft Graph API. This document describes Phase 1, which establishes a working proof of concept covering authentication and basic mail access.

## Commands

| Command | Description |
|---|---|
| `msgraph --version` | Print the CLI version |
| `msgraph auth login` | Authenticate via OAuth2 device code flow |
| `msgraph auth logout` | Clear stored credentials from keyring |
| `msgraph mail list` | List the 20 most recent emails as JSON |

## Architecture

### Source Layout

```
src/
  main.rs               CLI entry point, clap command tree
  config.rs             Resolve client_id (env var → config file)
  commands/
    auth.rs             auth login / auth logout handlers
    mail.rs             mail list handler
  auth/
    device_code.rs      OAuth2 device code flow via ureq
    token_store.rs      Keyring-based token persistence
  graph/
    client.rs           GraphClient with automatic token refresh
    mail.rs             GET /me/messages → minimal MailMessage structs
```

### Dependencies

| Crate | Purpose |
|---|---|
| `clap 4` | CLI parsing with derive macros |
| `keyring 3` | OS credential store (Windows Credential Manager, macOS Keychain, libsecret on Linux) |
| `ureq 2` | HTTP client for OAuth2 endpoints and Graph API |
| `serde` + `serde_json` | Serialization / JSON output |
| `chrono` | Token expiry tracking |

## Authentication Flow

Authentication uses the [OAuth2 Device Code Flow](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code), which is ideal for CLI tools:

1. CLI requests a device code from Microsoft's identity platform
2. User is shown a URL and short code to enter in a browser
3. CLI polls for completion — user signs in on any device
4. On success, access token + refresh token are stored in the OS keyring
5. Subsequent commands load the token automatically, refreshing if expired

**Supported account types:** Work/school accounts (AAD) and personal Microsoft accounts.

## Configuration

Client ID is resolved in this order:
1. `MSGRAPH_CLIENT_ID` environment variable
2. `~/.config/msgraph/config.toml`:
   ```toml
   [auth]
   client_id = "<your-azure-app-registration-client-id>"
   ```

## Azure App Registration Requirements

Register an app in [Entra ID (Azure Portal)](https://portal.azure.com):

- **Supported account types:** Accounts in any organizational directory and personal Microsoft accounts
- **Platform:** Mobile and desktop applications
- **Redirect URI:** `http://localhost`
- **API permissions (delegated):**
  - `User.Read`
  - `Mail.Read`
  - `offline_access` (for refresh tokens)

## Mail Output Format

`msgraph mail list` prints a JSON array to stdout:

```json
[
  {
    "id": "AAMkAGI...",
    "subject": "Meeting tomorrow",
    "from": "alice@example.com",
    "received_date_time": "2026-03-20T14:30:00Z",
    "body_preview": "Hi, just wanted to confirm...",
    "is_read": false
  }
]
```

Only essential fields are included to minimize token consumption by AI agents.
