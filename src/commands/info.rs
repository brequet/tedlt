use crate::AppError;
use crate::cli::InfoCommand;
use crate::config::ResolvedConfig;
use crate::jira::JiraClient;
use serde::Serialize;
use tracing::warn;

pub async fn handle_command(
    cmd: InfoCommand,
    client: &JiraClient,
    _config: &ResolvedConfig,
) -> Result<(), AppError> {
    use crate::cli::InfoSubCommand::*;

    match cmd.subcmd {
        Project { key } => {
            let project = client.get_project(key).await?;
            print_output(&project, cmd.json)?;
        }
        Ticket { key: _ } => {
            warn!("TODO: implement this.")
            // let ticket = client.get_ticket(&key).await?;
            // print_output(ticket, cmd.json)?;
        }
        Epics {
            project_key: _,
            board_id: _,
        } => {
            warn!("TODO: implement this.")
            // if let Some(board_id) = board_id {
            //     let epics = client.get_epics_by_board(board_id).await?;
            //     print_output(epics, cmd.json)?;
            // } else if let Some(project_key) = project_key {
            //     let boards = client.get_boards_by_project(&project_key).await?;
            //     if boards.is_empty() {
            //         warn!("No boards found for project '{}'", project_key);
            //         return Ok(());
            //     }

            //     for board in boards {
            //         println!("Epics for board: {} ({})", board.name, board.id);
            //         let epics = client.get_epics_by_board(board.id).await?;
            //         print_output(epics, cmd.json)?;
            //     }
            // }
        }
        Boards { project: _ } => {
            warn!("TODO: implement this.")
            // let boards = client.get_boards(project.as_deref()).await?;
            // print_output(boards, cmd.json)?;
        }
        Fields {
            project_key: _,
            issue_type: _,
        } => {
            warn!("TODO: implement this.")
            // let fields = client.get_fields(&project_key, &issue_type).await?;
            // print_output(fields, cmd.json)?;
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
