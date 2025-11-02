use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

mod cli;
mod config;
mod env;
mod jira;

use cli::Args;
use config::Config;
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

    let config_path = dirs::home_dir()
        .ok_or_else(|| config::ConfigError::NoHomeDir)?
        .join("tedlt.toml");

    let config_exists = config_path.exists();
    let config = Config::load()?;

    if !config_exists {
        warn!(
            "📝 Please edit your configuration file at: {}",
            config_path.display()
        );
        warn!("   Update jira_url and project_key with your Jira settings");
        info!("Configuration file created. Please edit it first.");
        return Ok(());
    }

    let credentials = Credentials::load()?;

    info!("Creating Jira ticket in project: {}", config.project_key);

    let client = JiraClient::new(
        config.jira_url.clone(),
        config.project_key,
        credentials.api_token,
        credentials.email,
    );

    let ticket = client.create_ticket(&args.title).await?;

    info!("Ticket created successfully:");
    println!("{}/browse/{}", config.jira_url, ticket.key);

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
