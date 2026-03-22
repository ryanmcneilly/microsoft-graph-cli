# microsoft-graph-cli

Provides a single CLI for the Microsoft Graph API.  It is primarily designed for AI agents.  It uses the Microsoft Entra ID device authentication flow for getting the token.  Inspiration comes from [googleworkspace/cli](https://github.com/googleworkspace/cli).

> [!IMPORTANT]
> This is a **WORK IN PROGRESS**, is incomplete and likely has lots of bugs.  **NOT** officially supported at all at this time.
> Only works with Microsoft Entra ID work/school tenants for now.

## Contents

- [Quick Start](#quick-start)
- [App Registration](#app-registration)
- [Environment Setup](#client-environment-setup)

## App Registration

An app needs to be registered in Microsoft Entra ID to enable the login flow:

TBD

## Client Environment Setup

Add the following environment variables to enable auth login flow:

1. MSGRAPH_TENANT_ID - the Directory (tenant) ID found in the "Overview" panel in App Registrations blade.
2. MSGRAPH_CLIENT_ID - the Application (client) ID found in the "Overview" panel in the App Registrations blade.

## Quick Start

```bash
msgraph --version
msgraph auth login      # Gets you authenticated to Microsoft Entra ID
msgraph mail list       # Lists the most recent 20 emails
msgraph auth logout     # Removes the saved refresh token effectively logging you out
```