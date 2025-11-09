use crate::AppError;
use crate::cli::InfoCommand;
use crate::config::ResolvedConfig;
use crate::jira::JiraClient;
use serde::Serialize;
use tracing::warn;

pub async fn handle_command(
    cmd: InfoCommand,
    client: &JiraClient,
    config: &ResolvedConfig,
) -> Result<(), AppError> {
    use crate::cli::InfoSubCommand::*;

    match cmd.subcmd {
        Project { key } => {
            warn!("TODO: implement");
            // Placeholder: Replace with actual API call
            // let metadata = client.get_project_metadata(&key).await?;
            // print_output(&metadata, cmd.json)?;
        }
        Epics { key } => {
            warn!("TODO: implement");
            // Placeholder: Replace with actual API call
            // println!(
            //     "Fetching epics for project '{}'. This will be implemented next.",
            //     key
            // );
            // let epics = client.get_project_epics(&key).await?;
            // print_output(&epics, cmd.json)?;
        }
        Ticket { key } => {
            warn!("TODO: implement");
            // Placeholder: Replace with actual API call
            // let issue = client.get_issue(&key).await?;
            // print_output(&issue, cmd.json)?;
        }
        Boards { project } => {
            warn!("TODO: implement");
            // Placeholder: Replace with actual API call
            // println!(
            //     "Fetching boards (project: {:?}). This will be implemented next.",
            //     project
            // );
            // let boards = client.get_boards(project.as_deref()).await?;
            // print_output(&boards, cmd.json)?;
        }
    }

    Ok(())
}

/// Helper function to print data either as pretty-printed JSON or a
/// human-readable format.
///
/// # Arguments
///
/// * `data` - The data to print, which must implement `Serialize`.
/// * `is_json` - A boolean flag indicating whether to print as JSON.
fn print_output<T>(data: &T, is_json: bool) -> Result<(), AppError>
where
    T: Serialize + ?Sized,
{
    if is_json {
        let json = serde_json::to_string_pretty(data).map_err(|e| AppError::Json(e.to_string()))?;
        println!("{}", json);
    } else {
        // In the future, we will add human-readable formatters here.
        // For now, we default to JSON for simplicity.
        let json = serde_json::to_string_pretty(data).map_err(|e| AppError::Json(e.to_string()))?;
        println!("{}", json);
    }
    Ok(())
}
