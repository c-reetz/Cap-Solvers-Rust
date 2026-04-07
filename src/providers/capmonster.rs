//! CapMonster API implementation.

use crate::error::{Error, Result};
use crate::types::{Balance, CaptchaSolver, TaskResult, TaskStatus, TaskType};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const CAPMONSTER_API_URL: &str = "https://api.capmonster.cloud";

/// CapMonster API client.
///
/// Provides access to the [CapMonster Cloud](https://capmonster.cloud/) captcha solving service.
///
/// # Examples
///
/// ```no_run
/// use cap_solvers::{CapMonster, CaptchaSolver, TaskType};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let solver = CapMonster::new("YOUR_API_KEY");
///
/// // Check balance
/// let balance = solver.get_balance().await?;
/// println!("Balance: ${}", balance.balance);
///
/// // Solve a captcha
/// let task_id = solver.create_task(TaskType::ReCaptchaV2Proxyless {
///     website_url: "https://example.com".to_string(),
///     website_key: "6Le-wvkSAAAAAPBMRTvw0Q4Muexq9bi0DJwx_mJ-".to_string(),
///     is_invisible: Some(false),
/// }).await?;
///
/// let result = solver.poll_task_result(&task_id, 120, 5).await?;
/// println!("Solution: {:?}", result.solution);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct CapMonster {
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
    cost: Option<String>,
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

impl CapMonster {
    /// Create a new CapMonster client.
    ///
    /// # Arguments
    /// * `api_key` - Your CapMonster API key
    ///
    /// # Examples
    ///
    /// ```
    /// use cap_solvers::CapMonster;
    ///
    /// let solver = CapMonster::new("your-api-key-here");
    /// ```
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: Client::new(),
        }
    }

    fn task_to_json(&self, task: TaskType) -> Result<serde_json::Value> {
        let json = match task {
            TaskType::ImageToText {
                website_url: _,
                body,
                module: _,
                images: _,
            } => {
                serde_json::json!({
                    "type": "ImageToTextTask",
                    "body": body,
                })
            }
            TaskType::ReCaptchaV2Proxyless {
                website_url,
                website_key,
                is_invisible,
            } => {
                let mut json = serde_json::json!({
                    "type": "NoCaptchaTaskProxyless",
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
                    "type": "NoCaptchaTask",
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
                    "type": "RecaptchaV3TaskProxyless",
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
                    "type": "RecaptchaV3Task",
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
                    "type": "RecaptchaV3EnterpriseTaskProxyless",
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
                    "type": "RecaptchaV3EnterpriseTask",
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
impl CaptchaSolver for CapMonster {
    async fn create_task(&self, task: TaskType) -> Result<String> {
        let task_json = self.task_to_json(task)?;
        let request = CreateTaskRequest {
            client_key: self.api_key.clone(),
            task: task_json,
        };

        let response = self
            .client
            .post(format!("{}/createTask", CAPMONSTER_API_URL))
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
            .post(format!("{}/getTaskResult", CAPMONSTER_API_URL))
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

        let cost = response.cost.and_then(|c| c.parse::<f64>().ok());

        Ok(TaskResult {
            task_id: task_id.to_string(),
            status,
            solution,
            error: response.error_description,
            cost,
        })
    }

    async fn get_balance(&self) -> Result<Balance> {
        let request = GetBalanceRequest {
            client_key: self.api_key.clone(),
        };

        let response = self
            .client
            .post(format!("{}/getBalance", CAPMONSTER_API_URL))
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
    fn test_capmonster_new() {
        let solver = CapMonster::new("test_key");
        assert_eq!(solver.api_key, "test_key");
    }

    #[test]
    fn test_task_to_json() {
        let solver = CapMonster::new("test_key");

        let task = TaskType::ReCaptchaV2Proxyless {
            website_url: "https://example.com".to_string(),
            website_key: "key123".to_string(),
            is_invisible: Some(false),
        };
        let json = solver.task_to_json(task).unwrap();
        assert_eq!(json["type"], "NoCaptchaTaskProxyless");
        assert_eq!(json["websiteURL"], "https://example.com");
    }
}
