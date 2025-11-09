use crate::AppError;
use crate::cli::CreateCommand;
use crate::config::ResolvedConfig;
use crate::jira::JiraClient;
use tracing::info;

pub async fn handle_command(
    cmd: CreateCommand,
    client: &JiraClient,
    config: &ResolvedConfig,
) -> Result<(), AppError> {
    let ticket = client
        .create_ticket(&cmd.title, config.fields.clone())
        .await?;

    info!("Ticket created successfully:");
    println!("{}/browse/{}", config.jira_url, ticket.key);

    Ok(())
}
