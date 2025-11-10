use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use thiserror::Error;
use tracing::debug;

#[derive(Error, Debug)]
pub enum JiraError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Failed to create ticket: {0}")]
    CreateTicket(String),

    #[error("Failed to get ticket: {0}")]
    GetTicket(String),

    #[error("Failed to get epics: {0}")]
    GetEpics(String),

    #[error("Failed to get boards: {0}")]
    GetBoard(String),

    #[error("Failed to get project: {0}")]
    GetProject(String),
}

#[derive(Debug, Serialize)]
struct IssueFields {
    project: Project,
    summary: String,
}

#[derive(Debug, Serialize)]
struct Project {
    key: String,
}

#[derive(Debug, Deserialize)]
struct CreateIssueResponse {
    key: String,
    #[serde(rename = "self")]
    self_url: String,
}

pub struct JiraClient {
    client: Client,
    base_url: String,
    project_key: String,
    api_token: String,
    email: String,
}

#[derive(Debug)]
pub struct TicketInfo {
    pub key: String,
    #[allow(dead_code)]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JiraProject {
    pub id: String,
    pub key: String,
    pub name: String,
    #[serde(rename = "issueTypes")]
    pub issue_types: Vec<IssueType>,
    pub components: Vec<Component>,
    pub versions: Vec<Version>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueType {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "subtask")]
    pub is_subtask: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Component {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub id: String,
    pub name: String,
    pub released: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Epic {
    pub id: u64,
    pub key: String,
    pub name: String,
    pub summary: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub board_type: String,
    #[serde(rename = "location")]
    pub project: Option<ProjectInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectInfo {
    #[serde(rename = "projectKey")]
    pub project_key: String,
}

impl JiraClient {
    pub fn new(base_url: String, project_key: String, api_token: String, email: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            project_key,
            api_token,
            email,
        }
    }

    pub async fn create_ticket(
        &self,
        title: &str,
        additional_fields: Option<Value>,
    ) -> Result<TicketInfo, JiraError> {
        let url = format!("{}/rest/api/3/issue", self.base_url);

        debug!(
            r#"Creating Jira issue:
            - Jira instance: {}
            - Project key: {}
            "#,
            self.base_url, self.project_key
        );

        let base_fields = IssueFields {
            project: Project {
                key: self.project_key.clone(),
            },
            summary: title.to_string(),
        };

        let mut fields_value = serde_json::to_value(base_fields).map_err(|e| {
            JiraError::CreateTicket(format!("Failed to serialize base fields: {}", e))
        })?;

        if let Some(additional) = additional_fields {
            if let (Some(fields_map), Some(additional_map)) =
                (fields_value.as_object_mut(), additional.as_object())
            {
                fields_map.extend(additional_map.clone());
            }
        }

        let request_body = json!({ "fields": fields_value });

        debug!(
            "Jira request body: {}",
            serde_json::to_string_pretty(&request_body).unwrap_or_default()
        );

        let response = self
            .client
            .post(&url)
            .basic_auth(&self.email, Some(&self.api_token))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(JiraError::CreateTicket(format!(
                "Status: {}, Body: {}",
                status, error_text
            )));
        }

        let create_response: CreateIssueResponse = response.json().await?;

        Ok(TicketInfo {
            key: create_response.key,
            url: create_response.self_url,
        })
    }

    pub async fn get_project(&self, project_key: Option<String>) -> Result<JiraProject, JiraError> {
        let project_key = project_key
            .or_else(|| Some(self.project_key.clone()))
            .ok_or_else(|| {
                JiraError::GetProject(
                    "Missing project key from input or loaded configuration.".to_string(),
                )
            })?;

        let response = self
            .client
            .get(format!(
                "{}/rest/api/3/project/{}",
                self.base_url, project_key
            ))
            .basic_auth(&self.email, Some(&self.api_token))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(JiraError::GetProject(format!(
                "Status: {}, Body: {}",
                status, error_text
            )));
        }

        let project = response.json::<JiraProject>().await?;
        Ok(project)
    }

    pub async fn get_ticket(&self, ticket_key: String) -> Result<Value, JiraError> {
        let response = self
            .client
            .get(format!("{}/rest/api/3/issue/{}", self.base_url, ticket_key))
            .basic_auth(&self.email, Some(&self.api_token))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(JiraError::GetTicket(format!(
                "Status: {}, Body: {}",
                status, error_text
            )));
        }

        let project = response.json::<Value>().await?;
        Ok(project)
    }

    pub async fn get_epics_by_board(&self, board_id: u64) -> Result<Vec<Epic>, JiraError> {
        let response = self
            .client
            .get(format!(
                "{}/rest/agile/1.0/board/{}/epic",
                self.base_url, board_id
            ))
            .basic_auth(&self.email, Some(&self.api_token))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(JiraError::GetEpics(format!(
                "Status: {}, Body: {}",
                status, error_text
            )));
        }

        #[derive(Deserialize)]
        struct EpicResponse {
            values: Vec<Epic>,
        }

        let epics = response.json::<EpicResponse>().await?;
        Ok(epics.values)
    }

    pub async fn get_boards(&self, project_key: Option<&str>) -> Result<Vec<Board>, JiraError> {
        let project_key = project_key.or_else(|| Some(self.project_key.as_ref()));

        let mut url = format!("{}/rest/agile/1.0/board", self.base_url);
        if let Some(key) = project_key {
            url.push_str(&format!("?projectKeyOrId={}", key));
        }

        let response = self
            .client
            .get(url)
            .basic_auth(&self.email, Some(&self.api_token))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(JiraError::GetBoard(format!(
                "Status: {}, Body: {}",
                status, error_text
            )));
        }

        #[derive(Deserialize)]
        struct BoardResponse {
            values: Vec<Board>,
        }

        let boards = response.json::<BoardResponse>().await?;
        Ok(boards.values)
    }
}
