use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const STORY_ISSUE_TYPE: &str = "10004";

#[derive(Error, Debug)]
pub enum JiraError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Failed to create ticket: {0}")]
    CreateTicket(String),
}

#[derive(Debug, Serialize)]
struct CreateIssueRequest {
    fields: IssueFields,
}

#[derive(Debug, Serialize)]
struct IssueFields {
    project: Project,
    summary: String,
    issuetype: IssueType,
}

#[derive(Debug, Serialize)]
struct Project {
    key: String,
}

#[derive(Debug, Serialize)]
struct IssueType {
    id: String,
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

    pub async fn create_ticket(&self, title: &str) -> Result<TicketInfo, JiraError> {
        let url = format!("{}/rest/api/3/issue", self.base_url);

        let request = CreateIssueRequest {
            fields: IssueFields {
                project: Project {
                    key: self.project_key.clone(),
                },
                summary: title.to_string(),
                issuetype: IssueType {
                    id: STORY_ISSUE_TYPE.to_string(),
                },
            },
        };

        let response = self
            .client
            .post(&url)
            .basic_auth(&self.email, Some(&self.api_token))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&request)
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
}

#[derive(Debug)]
pub struct TicketInfo {
    pub key: String,
    #[allow(dead_code)]
    pub url: String,
}
