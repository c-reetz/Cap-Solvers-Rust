//! CapSolver API implementation.

use crate::error::{Error, Result};
use crate::types::{Balance, CaptchaSolver, TaskResult, TaskStatus, TaskType};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const CAPSOLVER_API_URL: &str = "https://api.capsolver.com";

/// CapSolver client
#[derive(Debug, Clone)]
pub struct CapSolver {
    api_key: String,
    client: Client,
}

#[derive(Serialize)]
struct CreateTaskRequest {
    #[serde(rename = "clientKey")]
    client_key: String,
    task: serde_json::Value,
}

#[derive(Deserialize)]
struct CreateTaskResponse {
    #[serde(rename = "errorId")]
    error_id: i32,
    #[serde(rename = "errorDescription")]
    error_description: Option<String>,
    #[serde(rename = "taskId")]
    task_id: Option<String>,
}

#[derive(Serialize)]
struct GetTaskResultRequest {
    #[serde(rename = "clientKey")]
    client_key: String,
    #[serde(rename = "taskId")]
    task_id: String,
}

#[derive(Deserialize)]
struct GetTaskResultResponse {
    #[serde(rename = "errorId")]
    error_id: i32,
    #[serde(rename = "errorDescription")]
    error_description: Option<String>,
    status: Option<String>,
    solution: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct GetBalanceRequest {
    #[serde(rename = "clientKey")]
    client_key: String,
}

#[derive(Deserialize)]
struct GetBalanceResponse {
    #[serde(rename = "errorId")]
    error_id: i32,
    #[serde(rename = "errorDescription")]
    error_description: Option<String>,
    balance: Option<f64>,
}

impl CapSolver {
    /// Create a new CapSolver client
    ///
    /// # Arguments
    /// * `api_key` - Your CapSolver API key
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: Client::new(),
        }
    }

    fn task_to_json(&self, task: TaskType) -> Result<serde_json::Value> {
        let json = match task {
            TaskType::ImageToText { body } => {
                serde_json::json!({
                    "type": "ImageToTextTask",
                    "body": body,
                })
            }
            TaskType::ReCaptchaV2 {
                website_url,
                website_key,
                is_invisible,
            } => {
                let mut json = serde_json::json!({
                    "type": "ReCaptchaV2Task",
                    "websiteURL": website_url,
                    "websiteKey": website_key,
                });
                if let Some(invisible) = is_invisible {
                    json["isInvisible"] = serde_json::json!(invisible);
                }
                json
            }
            TaskType::ReCaptchaV3 {
                website_url,
                website_key,
                page_action,
                min_score,
            } => {
                let mut json = serde_json::json!({
                    "type": "ReCaptchaV3Task",
                    "websiteURL": website_url,
                    "websiteKey": website_key,
                    "pageAction": page_action,
                });
                if let Some(score) = min_score {
                    json["minScore"] = serde_json::json!(score);
                }
                json
            }
            TaskType::HCaptcha {
                website_url,
                website_key,
            } => {
                serde_json::json!({
                    "type": "HCaptchaTask",
                    "websiteURL": website_url,
                    "websiteKey": website_key,
                })
            }
            TaskType::FunCaptcha {
                website_url,
                website_public_key,
            } => {
                serde_json::json!({
                    "type": "FunCaptchaTask",
                    "websiteURL": website_url,
                    "websitePublicKey": website_public_key,
                })
            }
            TaskType::Custom { task_type, data } => {
                let mut json = serde_json::json!({
                    "type": task_type,
                });
                for (key, value) in data {
                    json[key] = value;
                }
                json
            }
        };
        Ok(json)
    }
}

#[async_trait]
impl CaptchaSolver for CapSolver {
    async fn create_task(&self, task: TaskType) -> Result<String> {
        let task_json = self.task_to_json(task)?;
        let request = CreateTaskRequest {
            client_key: self.api_key.clone(),
            task: task_json,
        };

        let response = self
            .client
            .post(format!("{}/createTask", CAPSOLVER_API_URL))
            .json(&request)
            .send()
            .await?
            .json::<CreateTaskResponse>()
            .await?;

        if response.error_id != 0 {
            return Err(Error::Api(
                response
                    .error_description
                    .unwrap_or_else(|| "Unknown error".to_string()),
            ));
        }

        response
            .task_id
            .ok_or_else(|| Error::Api("No task ID returned".to_string()))
    }

    async fn get_task_result(&self, task_id: &str) -> Result<TaskResult> {
        let request = GetTaskResultRequest {
            client_key: self.api_key.clone(),
            task_id: task_id.to_string(),
        };

        let response = self
            .client
            .post(format!("{}/getTaskResult", CAPSOLVER_API_URL))
            .json(&request)
            .send()
            .await?
            .json::<GetTaskResultResponse>()
            .await?;

        if response.error_id != 0 {
            return Err(Error::Api(
                response
                    .error_description
                    .unwrap_or_else(|| "Unknown error".to_string()),
            ));
        }

        let status = match response.status.as_deref() {
            Some("processing") => TaskStatus::Processing,
            Some("ready") => TaskStatus::Ready,
            Some("failed") => TaskStatus::Failed,
            _ => TaskStatus::Processing,
        };

        let solution = if let Some(serde_json::Value::Object(map)) = response.solution {
            Some(map.into_iter().collect())
        } else {
            None
        };

        Ok(TaskResult {
            task_id: task_id.to_string(),
            status,
            solution,
            error: response.error_description,
            cost: None,
        })
    }

    async fn get_balance(&self) -> Result<Balance> {
        let request = GetBalanceRequest {
            client_key: self.api_key.clone(),
        };

        let response = self
            .client
            .post(format!("{}/getBalance", CAPSOLVER_API_URL))
            .json(&request)
            .send()
            .await?
            .json::<GetBalanceResponse>()
            .await?;

        if response.error_id != 0 {
            return Err(Error::Api(
                response
                    .error_description
                    .unwrap_or_else(|| "Unknown error".to_string()),
            ));
        }

        Ok(Balance {
            balance: response.balance.unwrap_or(0.0),
            currency: Some("USD".to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capsolver_new() {
        let solver = CapSolver::new("test_key");
        assert_eq!(solver.api_key, "test_key");
    }

    #[test]
    fn test_task_to_json() {
        let solver = CapSolver::new("test_key");

        let task = TaskType::ImageToText {
            body: "base64data".to_string(),
        };
        let json = solver.task_to_json(task).unwrap();
        assert_eq!(json["type"], "ImageToTextTask");
        assert_eq!(json["body"], "base64data");
    }
}
