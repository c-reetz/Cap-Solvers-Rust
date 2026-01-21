//! Core types and traits for the cap_solvers crate.

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// The balance amount
    pub balance: f64,
    /// Currency (e.g., "USD")
    pub currency: Option<String>,
}

/// Task status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Task is being processed
    Processing,
    /// Task is ready with a result
    Ready,
    /// Task failed
    Failed,
}

/// Task result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    /// Task ID
    pub task_id: String,
    /// Status of the task
    pub status: TaskStatus,
    /// Solution data (if ready)
    pub solution: Option<HashMap<String, serde_json::Value>>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Cost of the task
    pub cost: Option<f64>,
}

/// Task types supported by captcha solvers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TaskType {
    /// Image to text captcha
    ImageToText {
        /// Base64 encoded image
        body: String,
    },
    /// ReCaptcha v2
    ReCaptchaV2 {
        /// Website URL
        website_url: String,
        /// Website key
        website_key: String,
        /// Is invisible
        #[serde(skip_serializing_if = "Option::is_none")]
        is_invisible: Option<bool>,
    },
    /// ReCaptcha v3
    ReCaptchaV3 {
        /// Website URL
        website_url: String,
        /// Website key
        website_key: String,
        /// Page action
        page_action: String,
        /// Minimum score (0.1 - 0.9)
        #[serde(skip_serializing_if = "Option::is_none")]
        min_score: Option<f64>,
    },
    /// hCaptcha
    HCaptcha {
        /// Website URL
        website_url: String,
        /// Website key
        website_key: String,
    },
    /// FunCaptcha
    FunCaptcha {
        /// Website URL
        website_url: String,
        /// Website public key
        website_public_key: String,
    },
    /// Generic task with custom data
    Custom {
        /// Task type name
        task_type: String,
        /// Task data
        data: HashMap<String, serde_json::Value>,
    },
}

/// Main trait for captcha solver implementations
#[async_trait::async_trait]
pub trait CaptchaSolver: Send + Sync {
    /// Create a new captcha solving task
    ///
    /// # Arguments
    /// * `task` - The task type and parameters
    ///
    /// # Returns
    /// The task ID
    async fn create_task(&self, task: TaskType) -> Result<String>;

    /// Get the result of a task
    ///
    /// # Arguments
    /// * `task_id` - The task ID returned by create_task
    ///
    /// # Returns
    /// The task result
    async fn get_task_result(&self, task_id: &str) -> Result<TaskResult>;

    /// Poll for task result with timeout
    ///
    /// # Arguments
    /// * `task_id` - The task ID
    /// * `timeout_secs` - Maximum time to wait in seconds
    /// * `poll_interval_secs` - Time between polls in seconds
    ///
    /// # Returns
    /// The task result when ready
    async fn poll_task_result(
        &self,
        task_id: &str,
        timeout_secs: u64,
        poll_interval_secs: u64,
    ) -> Result<TaskResult> {
        let start = std::time::Instant::now();
        loop {
            let result = self.get_task_result(task_id).await?;

            match result.status {
                TaskStatus::Ready => return Ok(result),
                TaskStatus::Failed => return Ok(result),
                TaskStatus::Processing => {
                    if start.elapsed().as_secs() >= timeout_secs {
                        return Err(crate::error::Error::TaskTimeout);
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(poll_interval_secs)).await;
                }
            }
        }
    }

    /// Get account balance
    ///
    /// # Returns
    /// Balance information
    async fn get_balance(&self) -> Result<Balance>;
}
