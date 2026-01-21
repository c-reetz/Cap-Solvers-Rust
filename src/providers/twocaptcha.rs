//! 2Captcha API implementation.

use crate::error::{Error, Result};
use crate::types::{Balance, CaptchaSolver, TaskResult, TaskStatus, TaskType};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const TWOCAPTCHA_API_URL: &str = "https://2captcha.com";

/// 2Captcha client
#[derive(Debug, Clone)]
pub struct TwoCaptcha {
    api_key: String,
    client: Client,
}

#[derive(Serialize)]
struct CreateTaskRequest {
    key: String,
    method: String,
    #[serde(flatten)]
    params: HashMap<String, String>,
    json: i32,
}

#[derive(Deserialize)]
struct CreateTaskResponse {
    status: i32,
    request: Option<String>,
    error_text: Option<String>,
}

#[derive(Deserialize)]
struct GetTaskResultResponse {
    status: i32,
    request: Option<String>,
    error_text: Option<String>,
}

#[derive(Deserialize)]
struct GetBalanceResponse {
    status: i32,
    request: Option<String>,
    error_text: Option<String>,
}

impl TwoCaptcha {
    /// Create a new 2Captcha client
    ///
    /// # Arguments
    /// * `api_key` - Your 2Captcha API key
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: Client::new(),
        }
    }

    fn task_to_params(&self, task: TaskType) -> Result<(String, HashMap<String, String>)> {
        let (method, params) = match task {
            TaskType::ImageToText { body } => {
                let mut params = HashMap::new();
                params.insert("body".to_string(), body);
                ("base64".to_string(), params)
            }
            TaskType::ReCaptchaV2 {
                website_url,
                website_key,
                is_invisible,
            } => {
                let mut params = HashMap::new();
                params.insert("googlekey".to_string(), website_key);
                params.insert("pageurl".to_string(), website_url);
                if let Some(invisible) = is_invisible {
                    params.insert(
                        "invisible".to_string(),
                        if invisible { "1" } else { "0" }.to_string(),
                    );
                }
                ("userrecaptcha".to_string(), params)
            }
            TaskType::ReCaptchaV3 {
                website_url,
                website_key,
                page_action,
                min_score,
            } => {
                let mut params = HashMap::new();
                params.insert("googlekey".to_string(), website_key);
                params.insert("pageurl".to_string(), website_url);
                params.insert("action".to_string(), page_action);
                if let Some(score) = min_score {
                    params.insert("min_score".to_string(), score.to_string());
                }
                params.insert("version".to_string(), "v3".to_string());
                ("userrecaptcha".to_string(), params)
            }
            TaskType::HCaptcha {
                website_url,
                website_key,
            } => {
                let mut params = HashMap::new();
                params.insert("sitekey".to_string(), website_key);
                params.insert("pageurl".to_string(), website_url);
                ("hcaptcha".to_string(), params)
            }
            TaskType::FunCaptcha {
                website_url,
                website_public_key,
            } => {
                let mut params = HashMap::new();
                params.insert("publickey".to_string(), website_public_key);
                params.insert("pageurl".to_string(), website_url);
                ("funcaptcha".to_string(), params)
            }
            TaskType::Custom { task_type, data } => {
                let mut params = HashMap::new();
                for (key, value) in data {
                    if let serde_json::Value::String(s) = value {
                        params.insert(key, s);
                    } else {
                        params.insert(key, value.to_string());
                    }
                }
                (task_type, params)
            }
        };
        Ok((method, params))
    }
}

#[async_trait]
impl CaptchaSolver for TwoCaptcha {
    async fn create_task(&self, task: TaskType) -> Result<String> {
        let (method, params) = self.task_to_params(task)?;
        let request = CreateTaskRequest {
            key: self.api_key.clone(),
            method,
            params,
            json: 1,
        };

        let response = self
            .client
            .post(format!("{}/in.php", TWOCAPTCHA_API_URL))
            .form(&request)
            .send()
            .await?
            .json::<CreateTaskResponse>()
            .await?;

        if response.status != 1 {
            return Err(Error::Api(
                response
                    .error_text
                    .unwrap_or_else(|| "Unknown error".to_string()),
            ));
        }

        response
            .request
            .ok_or_else(|| Error::Api("No task ID returned".to_string()))
    }

    async fn get_task_result(&self, task_id: &str) -> Result<TaskResult> {
        let response = self
            .client
            .get(format!("{}/res.php", TWOCAPTCHA_API_URL))
            .query(&[
                ("key", self.api_key.as_str()),
                ("action", "get"),
                ("id", task_id),
                ("json", "1"),
            ])
            .send()
            .await?
            .json::<GetTaskResultResponse>()
            .await?;

        if response.status != 1 {
            // Check if still processing
            if let Some(error) = &response.error_text {
                if error.contains("CAPCHA_NOT_READY") {
                    return Ok(TaskResult {
                        task_id: task_id.to_string(),
                        status: TaskStatus::Processing,
                        solution: None,
                        error: None,
                        cost: None,
                    });
                }
            }

            return Err(Error::Api(
                response
                    .error_text
                    .unwrap_or_else(|| "Unknown error".to_string()),
            ));
        }

        let mut solution = HashMap::new();
        if let Some(text) = response.request {
            solution.insert("text".to_string(), serde_json::json!(text));
        }

        Ok(TaskResult {
            task_id: task_id.to_string(),
            status: TaskStatus::Ready,
            solution: Some(solution),
            error: None,
            cost: None,
        })
    }

    async fn get_balance(&self) -> Result<Balance> {
        let response = self
            .client
            .get(format!("{}/res.php", TWOCAPTCHA_API_URL))
            .query(&[
                ("key", self.api_key.as_str()),
                ("action", "getbalance"),
                ("json", "1"),
            ])
            .send()
            .await?
            .json::<GetBalanceResponse>()
            .await?;

        if response.status != 1 {
            return Err(Error::Api(
                response
                    .error_text
                    .unwrap_or_else(|| "Unknown error".to_string()),
            ));
        }

        let balance = response
            .request
            .and_then(|b| b.parse::<f64>().ok())
            .unwrap_or(0.0);

        Ok(Balance {
            balance,
            currency: Some("USD".to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twocaptcha_new() {
        let solver = TwoCaptcha::new("test_key");
        assert_eq!(solver.api_key, "test_key");
    }

    #[test]
    fn test_task_to_params() {
        let solver = TwoCaptcha::new("test_key");

        let task = TaskType::HCaptcha {
            website_url: "https://example.com".to_string(),
            website_key: "key123".to_string(),
        };
        let (method, params) = solver.task_to_params(task).unwrap();
        assert_eq!(method, "hcaptcha");
        assert_eq!(params.get("sitekey").unwrap(), "key123");
        assert_eq!(params.get("pageurl").unwrap(), "https://example.com");
    }
}
