//! CapSolver API implementation.

use crate::error::{Error, Result};
use crate::types::{Balance, CaptchaSolver, TaskResult, TaskStatus, TaskType};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const CAPSOLVER_API_URL: &str = "https://api.capsolver.com";

/// CapSolver API client.
///
/// Provides access to the [CapSolver](https://www.capsolver.com/) captcha solving service.
///
/// # Examples
///
/// ```no_run
/// use cap_solvers::{CapSolver, CaptchaSolver, TaskType};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let solver = CapSolver::new("YOUR_API_KEY");
///
/// // Check balance
/// let balance = solver.get_balance().await?;
/// println!("Balance: ${}", balance.balance);
///
/// // Solve a captcha
/// let task_id = solver.create_task(TaskType::ImageToText {
///     website_url: None,
///     body: "base64_encoded_image".to_string(),
///     module: None,
///     images: None,
/// }).await?;
///
/// let result = solver.poll_task_result(&task_id, 120, 5).await?;
/// println!("Solution: {:?}", result.solution);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct CapSolver {
    api_key: String,
    client: Client,
    ready_task_results: Arc<Mutex<HashMap<String, TaskResult>>>,
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
    status: Option<String>,
    solution: Option<serde_json::Value>,
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
    /// Create a new CapSolver client.
    ///
    /// # Arguments
    /// * `api_key` - Your CapSolver API key
    ///
    /// # Examples
    ///
    /// ```
    /// use cap_solvers::CapSolver;
    ///
    /// let solver = CapSolver::new("your-api-key-here");
    /// ```
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: Client::new(),
            ready_task_results: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn task_to_json(&self, task: TaskType) -> Result<serde_json::Value> {
        let json = match task {
            TaskType::ImageToText {
                website_url,
                body,
                module,
                images,
            } => {
                let mut json = serde_json::json!({
                    "type": "ImageToTextTask",
                    "body": body,
                });
                if let Some(website_url) = website_url {
                    json["websiteURL"] = serde_json::json!(website_url);
                }
                if let Some(module) = module {
                    json["module"] = serde_json::json!(module);
                }
                if let Some(images) = images {
                    json["images"] = serde_json::json!(images);
                }
                json
            }
            TaskType::ReCaptchaV2Proxyless {
                website_url,
                website_key,
                is_invisible,
            } => {
                let mut json = serde_json::json!({
                    "type": "ReCaptchaV2TaskProxyless",
                    "websiteURL": website_url,
                    "websiteKey": website_key,
                });
                if let Some(invisible) = is_invisible {
                    json["isInvisible"] = serde_json::json!(invisible);
                }
                json
            }
            TaskType::ReCaptchaV2 {
                website_url,
                website_key,
                is_invisible,
                proxy,
            } => {
                let mut json = serde_json::json!({
                    "type": "ReCaptchaV2Task",
                    "websiteURL": website_url,
                    "websiteKey": website_key,
                    "proxyType": proxy.proxy_type,
                    "proxyAddress": proxy.proxy_address,
                    "proxyPort": proxy.proxy_port,
                });
                if let Some(invisible) = is_invisible {
                    json["isInvisible"] = serde_json::json!(invisible);
                }
                if let Some(login) = proxy.proxy_login {
                    json["proxyLogin"] = serde_json::json!(login);
                }
                if let Some(password) = proxy.proxy_password {
                    json["proxyPassword"] = serde_json::json!(password);
                }
                json
            }
            TaskType::ReCaptchaV3Proxyless {
                website_url,
                website_key,
                page_action,
                min_score,
            } => {
                let mut json = serde_json::json!({
                    "type": "ReCaptchaV3TaskProxyless",
                    "websiteURL": website_url,
                    "websiteKey": website_key,
                    "pageAction": page_action,
                });
                if let Some(score) = min_score {
                    json["minScore"] = serde_json::json!(score);
                }
                json
            }
            TaskType::ReCaptchaV3 {
                website_url,
                website_key,
                page_action,
                min_score,
                proxy,
            } => {
                let mut json = serde_json::json!({
                    "type": "ReCaptchaV3Task",
                    "websiteURL": website_url,
                    "websiteKey": website_key,
                    "pageAction": page_action,
                    "proxyType": proxy.proxy_type,
                    "proxyAddress": proxy.proxy_address,
                    "proxyPort": proxy.proxy_port,
                });
                if let Some(score) = min_score {
                    json["minScore"] = serde_json::json!(score);
                }
                if let Some(login) = proxy.proxy_login {
                    json["proxyLogin"] = serde_json::json!(login);
                }
                if let Some(password) = proxy.proxy_password {
                    json["proxyPassword"] = serde_json::json!(password);
                }
                json
            }
            TaskType::ReCaptchaV3EnterpriseProxyless {
                website_url,
                website_key,
                page_action,
                min_score,
                enterprise_payload,
            } => {
                let mut json = serde_json::json!({
                    "type": "ReCaptchaV3EnterpriseTaskProxyless",
                    "websiteURL": website_url,
                    "websiteKey": website_key,
                    "pageAction": page_action,
                });
                if let Some(score) = min_score {
                    json["minScore"] = serde_json::json!(score);
                }
                if let Some(payload) = enterprise_payload {
                    json["enterprisePayload"] = serde_json::json!(payload);
                }
                json
            }
            TaskType::ReCaptchaV3Enterprise {
                website_url,
                website_key,
                page_action,
                min_score,
                enterprise_payload,
                proxy,
            } => {
                let mut json = serde_json::json!({
                    "type": "ReCaptchaV3EnterpriseTask",
                    "websiteURL": website_url,
                    "websiteKey": website_key,
                    "pageAction": page_action,
                    "proxyType": proxy.proxy_type,
                    "proxyAddress": proxy.proxy_address,
                    "proxyPort": proxy.proxy_port,
                });
                if let Some(score) = min_score {
                    json["minScore"] = serde_json::json!(score);
                }
                if let Some(payload) = enterprise_payload {
                    json["enterprisePayload"] = serde_json::json!(payload);
                }
                if let Some(login) = proxy.proxy_login {
                    json["proxyLogin"] = serde_json::json!(login);
                }
                if let Some(password) = proxy.proxy_password {
                    json["proxyPassword"] = serde_json::json!(password);
                }
                json
            }
            TaskType::HCaptchaProxyless {
                website_url,
                website_key,
            } => {
                serde_json::json!({
                    "type": "HCaptchaTaskProxyless",
                    "websiteURL": website_url,
                    "websiteKey": website_key,
                })
            }
            TaskType::HCaptcha {
                website_url,
                website_key,
                proxy,
            } => {
                let mut json = serde_json::json!({
                    "type": "HCaptchaTask",
                    "websiteURL": website_url,
                    "websiteKey": website_key,
                    "proxyType": proxy.proxy_type,
                    "proxyAddress": proxy.proxy_address,
                    "proxyPort": proxy.proxy_port,
                });
                if let Some(login) = proxy.proxy_login {
                    json["proxyLogin"] = serde_json::json!(login);
                }
                if let Some(password) = proxy.proxy_password {
                    json["proxyPassword"] = serde_json::json!(password);
                }
                json
            }
            TaskType::FunCaptchaProxyless {
                website_url,
                website_public_key,
            } => {
                serde_json::json!({
                    "type": "FunCaptchaTaskProxyless",
                    "websiteURL": website_url,
                    "websitePublicKey": website_public_key,
                })
            }
            TaskType::FunCaptcha {
                website_url,
                website_public_key,
                proxy,
            } => {
                let mut json = serde_json::json!({
                    "type": "FunCaptchaTask",
                    "websiteURL": website_url,
                    "websitePublicKey": website_public_key,
                    "proxyType": proxy.proxy_type,
                    "proxyAddress": proxy.proxy_address,
                    "proxyPort": proxy.proxy_port,
                });
                if let Some(login) = proxy.proxy_login {
                    json["proxyLogin"] = serde_json::json!(login);
                }
                if let Some(password) = proxy.proxy_password {
                    json["proxyPassword"] = serde_json::json!(password);
                }
                json
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

        let task_id = response
            .task_id
            .ok_or_else(|| Error::Api("No task ID returned".to_string()))?;

        if response.status.as_deref() == Some("ready") {
            let solution = if let Some(serde_json::Value::Object(map)) = response.solution {
                Some(map.into_iter().collect())
            } else {
                None
            };

            self.ready_task_results
                .lock()
                .map_err(|_| Error::Api("Ready task result cache lock poisoned".to_string()))?
                .insert(
                    task_id.clone(),
                    TaskResult {
                        task_id: task_id.clone(),
                        status: TaskStatus::Ready,
                        solution,
                        error: response.error_description.clone(),
                        cost: None,
                    },
                );
        }

        Ok(task_id)
    }

    async fn get_task_result(&self, task_id: &str) -> Result<TaskResult> {
        if let Some(result) = self
            .ready_task_results
            .lock()
            .map_err(|_| Error::Api("Ready task result cache lock poisoned".to_string()))?
            .get(task_id)
            .cloned()
        {
            return Ok(result);
        }

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
            website_url: Some("https://example.com".to_string()),
            body: "base64data".to_string(),
            module: Some("common".to_string()),
            images: Some(vec!["base64data-2".to_string()]),
        };
        let json = solver.task_to_json(task).unwrap();
        assert_eq!(json["type"], "ImageToTextTask");
        assert_eq!(json["body"], "base64data");
        assert_eq!(json["websiteURL"], "https://example.com");
        assert_eq!(json["module"], "common");
        assert_eq!(json["images"][0], "base64data-2");
    }
}
