use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;

mod cli;
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
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!("{}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), AppError> {
    let args = Args::parse_args();

    init_tracing(args.verbose);

    let cli_overrides = CliOverrides {
        jira_url: args.jira_url,
        project_key: args.project_key,
    };

    let config_file = ConfigFile::load()?;
    let resolved_config = config_file.resolve(args.profile, cli_overrides)?;

    let credentials = Credentials::load()?;

    let client = JiraClient::new(
        resolved_config.jira_url.clone(),
        resolved_config.project_key.clone(),
        credentials.api_token,
        credentials.email,
    );

    let ticket = client
        .create_ticket(&args.title, resolved_config.fields)
        .await?;

    info!("Ticket created successfully:");
    println!("{}/browse/{}", resolved_config.jira_url, ticket.key);

    Ok(())
}

fn init_tracing(verbose: bool) {
    let default_log_level = if verbose { "debug" } else { "info" };

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("tedlt={}", default_log_level)));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .without_time()
        .init();

    debug!("Verbose logging enabled.");
}
