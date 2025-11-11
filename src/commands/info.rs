use crate::AppError;
use crate::cli::InfoCommand;
use crate::jira::JiraClient;
use serde::Serialize;
use tracing::warn;

pub async fn handle_command(cmd: InfoCommand, client: &JiraClient) -> Result<(), AppError> {
    use crate::cli::InfoSubCommand::*;

    match cmd.subcmd {
        Project { key } => {
            let project = client.get_project(key).await?;
            print_output(&project)?;
        }
        Ticket { key } => {
            let ticket = client.get_ticket(&key).await?;
            print_output(&ticket)?;
        }
        Epics {
            project_key,
            board_id,
        } => {
            if let Some(board_id) = board_id {
                let epics = client.get_epics_by_board(board_id).await?;
                print_output(&epics)?;
            } else {
                let boards = client.get_boards(project_key.as_deref()).await?;
                if boards.is_empty() {
                    warn!("No boards found");
                    return Ok(());
                }

                for board in boards {
                    println!("Epics for board: {} ({})", board.name, board.id);
                    let epics = client.get_epics_by_board(board.id).await?;
                    print_output(&epics)?;
                }
            }
        }
        Boards { project } => {
            let boards = client.get_boards(project.as_deref()).await?;
            print_output(&boards)?;
        }
        Fields {
            project_key,
            issue_type,
        } => {
            let fields = client.get_fields(project_key, issue_type).await?;
            print_output(&fields)?;
        }
    }

    Ok(())
}

fn print_output<T>(data: &T) -> Result<(), AppError>
where
    T: Serialize + ?Sized,
{
    let json = serde_json::to_string_pretty(data).map_err(|e| AppError::Json(e.to_string()))?;
    println!("{}", json);
    Ok(())
}
