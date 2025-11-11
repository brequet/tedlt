use std::io;

use tracing::{debug, error};
use tracing_subscriber::EnvFilter;

mod cli;
mod commands;
mod config;
mod env;
mod jira;

use cli::Args;
use config::{CliOverrides, ConfigFile};
use env::Credentials;
use jira::JiraClient;

#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Environment error: {0}")]
    Env(#[from] env::EnvError),

    #[error("Jira API error: {0}")]
    Jira(#[from] jira::JiraError),

    #[error("JSON serialization/deserialization error: {0}")]
    Json(String),
}

#[tokio::main]
async fn main() {
    let args = Args::parse_args();
    init_tracing(args.verbose);

    if let Err(e) = run(args).await {
        error!("{}", e);
        std::process::exit(1);
    }
}

async fn run(args: Args) -> Result<(), AppError> {
    let config_file = ConfigFile::load()?;
    let cli_overrides = CliOverrides {
        jira_url: args.jira_url,
        project_key: args.project_key,
    };

    let profile_names = match &args.command {
        cli::Commands::Create(cmd) => cmd.profile.as_slice(),
        cli::Commands::Info(cmd) => cmd.profile.as_slice(),
    };

    let resolved_config = config_file.resolve(profile_names, cli_overrides)?;

    debug!("Resolved configuration: {:?}", resolved_config);

    let credentials = Credentials::load()?;

    let client = JiraClient::new(
        resolved_config.jira_url.clone(),
        resolved_config.project_key.clone(),
        credentials.api_token,
        credentials.email,
    );

    match args.command {
        cli::Commands::Create(cmd) => {
            commands::create::handle_command(cmd, &client, &resolved_config).await?
        }
        cli::Commands::Info(cmd) => commands::info::handle_command(cmd, &client).await?,
    }

    Ok(())
}

fn init_tracing(verbose: bool) {
    let default_log_level = if verbose { "debug" } else { "info" };

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(format!(
            "tedlt={},reqwest={}",
            default_log_level,
            if verbose { "debug" } else { "warn" }
        ))
    });

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .without_time()
        .with_writer(io::stderr)
        .init();

    debug!("Verbose logging enabled.");
}
