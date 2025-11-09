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
}
