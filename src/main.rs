mod auth;
mod commands;
mod config;
mod graph;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "msgraph", version, about = "Microsoft Graph API CLI for AI agents")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Authentication commands
    Auth {
        #[command(subcommand)]
        subcommand: AuthCommands,
    },
    /// Mail commands
    Mail {
        #[command(subcommand)]
        subcommand: MailCommands,
    },
}

#[derive(Subcommand)]
enum AuthCommands {
    /// Login with your Microsoft account
    Login,
    /// Logout and clear stored credentials
    Logout,
}

#[derive(Subcommand)]
enum MailCommands {
    /// List the 20 most recent emails
    List,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Auth { subcommand: AuthCommands::Login }) => commands::auth::handle_login(),
        Some(Commands::Auth { subcommand: AuthCommands::Logout }) => commands::auth::handle_logout(),
        Some(Commands::Mail { subcommand: MailCommands::List }) => commands::mail::handle_mail_list(),
        None => Ok(()),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
