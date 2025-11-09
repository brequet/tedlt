use tracing::{error, info};
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
    init_tracing();

    let args = Args::parse_args();

    let config_file = ConfigFile::load_from_home()?;

    let cli_overrides = CliOverrides {
        jira_url: args.jira_url,
        project_key: args.project_key,
    };

    let resolved_config = config_file.resolve(args.profile, cli_overrides)?;

    let credentials = Credentials::load()?;

    info!(
        "Creating Jira ticket in project: {}",
        resolved_config.project_key
    );

    let client = JiraClient::new(
        resolved_config.jira_url.clone(),
        resolved_config.project_key,
        credentials.api_token,
        credentials.email,
    );

    let ticket = client.create_ticket(&args.title).await?;

    info!("Ticket created successfully:");
    println!("{}/browse/{}", resolved_config.jira_url, ticket.key);

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .without_time()
        .init();
}
