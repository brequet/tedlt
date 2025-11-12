use tracing::info;

use crate::{
    AppError,
    cli::InitCommand,
    config::{ConfigFile, ProfileDef, get_home_config_file_path},
};

pub async fn handle_command(cmd: InitCommand) -> Result<(), AppError> {
    let cwd_path = get_home_config_file_path()?;
    if cwd_path.exists() && !cmd.force {
        println!("A config file already exists. Do you want to overwrite it? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborting.");
            return Ok(());
        }
    }

    let jira_url = get_jira_url(cmd.jira_url)?;
    let project_key = get_project_key(cmd.project_key)?;

    let config = ConfigFile {
        jira_url: Some(jira_url),
        project_key,
        properties: std::collections::HashMap::new(),
        profiles: {
            let mut profiles = std::collections::HashMap::new();
            profiles.insert(
                "default".to_string(),
                ProfileDef {
                    jira_url: None,
                    project_key: None,
                    fields: Some(serde_json::json!({})),
                    inherits: vec![], // TODO: should not appear in generated config
                },
            );
            profiles
        },
    };

    let config_content = serde_json::to_string_pretty(&config)
        .map_err(|e| AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

    std::fs::write(&cwd_path, config_content)?;

    info!("Configuration file created at {:?}", cwd_path);

    Ok(())
}

fn get_jira_url(jira_url: Option<String>) -> Result<String, AppError> {
    if let Some(url) = jira_url {
        return Ok(url);
    }

    println!("Enter the Jira URL (e.g., https://your-domain.atlassian.net): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let url = input.trim().to_string();
    if url.is_empty() {
        return Err(AppError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Jira URL cannot be empty",
        )));
    }
    Ok(url)
}

fn get_project_key(project_key: Option<String>) -> Result<Option<String>, AppError> {
    if let Some(key) = project_key {
        return Ok(Some(key));
    }

    println!("Enter the default project key (or leave empty to skip): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let key = input.trim().to_string();
    if key.is_empty() {
        return Ok(None);
    }
    Ok(Some(key))
}
