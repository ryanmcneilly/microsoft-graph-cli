# microsoft-graph-cli

The purpose of the microsoft-graph-cli is to provide a token efficient way for AI agents to interact with the Microsoft Graph API in place of an MCP server.  The project is written in Rust due to its memory and type safety.

The executable will is named msgraph.  

Builds target Windows, Mac OS and Linux.

Source code is in ./src

## RULES
The following rules MUST be adhered to 
1. *Do not* use the unsafe keyword.
2. *Do not* use an existing Microsoft Graph API crate.  Implement your own wrapper for the API.
3. *Always* ask before adding a new crate/dependency.  Controlling dependencies is important to this project.
4. *Always* use idomadic Rust patterns.