//! Core types and traits for the cap_solvers crate.

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Balance information returned from captcha solving services.
///
/// # Examples
///
/// ```no_run
/// use cap_solvers::{CapSolver, CaptchaSolver};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let solver = CapSolver::new("YOUR_API_KEY");
/// let balance = solver.get_balance().await?;
/// println!("Balance: ${}", balance.balance);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Balance {
    /// The balance amount
    pub balance: f64,
    /// Currency (e.g., "USD")
    pub currency: Option<String>,
}

/// Status of a captcha solving task.
///
/// # Examples
///
/// ```
/// use cap_solvers::TaskStatus;
///
/// let status = TaskStatus::Processing;
/// assert_eq!(status, TaskStatus::Processing);
///
/// // Default is Processing
/// let default_status = TaskStatus::default();
/// assert_eq!(default_status, TaskStatus::Processing);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    /// Task is being processed
    #[default]
    Processing,
    /// Task is ready with a result
    Ready,
    /// Task failed
    Failed,
}

/// Result of a captcha solving task.
///
/// Contains the task status, solution data (if ready), error message (if failed),
/// and cost information.
///
/// # Examples
///
/// ```no_run
/// use cap_solvers::{CapSolver, CaptchaSolver, TaskType};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let solver = CapSolver::new("YOUR_API_KEY");
/// let task_id = solver.create_task(TaskType::ImageToText {
///     website_url: None,
///     body: "base64_encoded_image".to_string(),
///     module: None,
///     images: None,
/// }).await?;
///
/// let result = solver.get_task_result(&task_id).await?;
/// println!("Task status: {:?}", result.status);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

/// Proxy configuration for captcha tasks that require proxy support.
///
/// Some captcha types require interaction through a proxy to simulate
/// a real user browsing from a specific location.
///
/// # Examples
///
/// ```
/// use cap_solvers::ProxyConfig;
///
/// let proxy = ProxyConfig {
///     proxy_type: "http".to_string(),
///     proxy_address: "proxy.example.com".to_string(),
///     proxy_port: 8080,
///     proxy_login: Some("username".to_string()),
///     proxy_password: Some("password".to_string()),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// Proxy type (e.g., "http", "https", "socks4", "socks5")
    pub proxy_type: String,
    /// Proxy address
    pub proxy_address: String,
    /// Proxy port
    pub proxy_port: u16,
    /// Proxy username (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_login: Option<String>,
    /// Proxy password (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_password: Option<String>,
}

/// Types of captcha tasks supported by the solving services.
///
/// Different captcha types require different parameters. Each variant
/// contains the necessary information to solve that specific captcha type.
///
/// # Examples
///
/// ```
/// use cap_solvers::TaskType;
///
/// // Simple image captcha
/// let task = TaskType::ImageToText {
///     website_url: None,
///     body: "base64_encoded_image_data".to_string(),
///     module: None,
///     images: None,
/// };
///
/// // ReCaptcha v2 without proxy
/// let task = TaskType::ReCaptchaV2Proxyless {
///     website_url: "https://example.com".to_string(),
///     website_key: "6Le-wvkSAAAAAPBMRTvw0Q4Muexq9bi0DJwx_mJ-".to_string(),
///     is_invisible: Some(false),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TaskType {
    /// Image to text captcha
    ImageToText {
        /// Optional page source URL to improve accuracy
        #[serde(skip_serializing_if = "Option::is_none")]
        website_url: Option<String>,
        /// Base64 encoded image
        body: String,
        /// Optional model/module name (for example `common` or `number`)
        #[serde(skip_serializing_if = "Option::is_none")]
        module: Option<String>,
        /// Optional batch image payload used by some modules such as `number`
        #[serde(skip_serializing_if = "Option::is_none")]
        images: Option<Vec<String>>,
    },
    /// ReCaptcha v2 (Proxyless)
    ReCaptchaV2Proxyless {
        /// Website URL
        website_url: String,
        /// Website key
        website_key: String,
        /// Is invisible
        #[serde(skip_serializing_if = "Option::is_none")]
        is_invisible: Option<bool>,
    },
    /// ReCaptcha v2 (with Proxy)
    ReCaptchaV2 {
        /// Website URL
        website_url: String,
        /// Website key
        website_key: String,
        /// Is invisible
        #[serde(skip_serializing_if = "Option::is_none")]
        is_invisible: Option<bool>,
        /// Proxy configuration
        proxy: ProxyConfig,
    },
    /// ReCaptcha v3 (Proxyless)
    ReCaptchaV3Proxyless {
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
    /// ReCaptcha v3 (with Proxy)
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
        /// Proxy configuration
        proxy: ProxyConfig,
    },
    /// ReCaptcha v3 Enterprise (Proxyless)
    ReCaptchaV3EnterpriseProxyless {
        /// Website URL
        website_url: String,
        /// Website key
        website_key: String,
        /// Page action
        page_action: String,
        /// Minimum score (0.1 - 0.9)
        #[serde(skip_serializing_if = "Option::is_none")]
        min_score: Option<f64>,
        /// Enterprise payload
        #[serde(skip_serializing_if = "Option::is_none")]
        enterprise_payload: Option<HashMap<String, serde_json::Value>>,
    },
    /// ReCaptcha v3 Enterprise (with Proxy)
    ReCaptchaV3Enterprise {
        /// Website URL
        website_url: String,
        /// Website key
        website_key: String,
        /// Page action
        page_action: String,
        /// Minimum score (0.1 - 0.9)
        #[serde(skip_serializing_if = "Option::is_none")]
        min_score: Option<f64>,
        /// Enterprise payload
        #[serde(skip_serializing_if = "Option::is_none")]
        enterprise_payload: Option<HashMap<String, serde_json::Value>>,
        /// Proxy configuration
        proxy: ProxyConfig,
    },
    /// hCaptcha (Proxyless)
    HCaptchaProxyless {
        /// Website URL
        website_url: String,
        /// Website key
        website_key: String,
    },
    /// hCaptcha (with Proxy)
    HCaptcha {
        /// Website URL
        website_url: String,
        /// Website key
        website_key: String,
        /// Proxy configuration
        proxy: ProxyConfig,
    },
    /// FunCaptcha (Proxyless)
    FunCaptchaProxyless {
        /// Website URL
        website_url: String,
        /// Website public key
        website_public_key: String,
    },
    /// FunCaptcha (with Proxy)
    FunCaptcha {
        /// Website URL
        website_url: String,
        /// Website public key
        website_public_key: String,
        /// Proxy configuration
        proxy: ProxyConfig,
    },
    /// Generic task with custom data
    Custom {
        /// Task type name
        task_type: String,
        /// Task data
        data: HashMap<String, serde_json::Value>,
    },
}

/// Main trait for captcha solver implementations.
///
/// All captcha solving providers implement this trait, providing a unified
/// interface for different services.
///
/// # Examples
///
/// ```no_run
/// use cap_solvers::{CapSolver, CaptchaSolver, TaskType};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let solver = CapSolver::new("YOUR_API_KEY");
///
/// // Create and solve a task
/// let task_id = solver.create_task(TaskType::ImageToText {
///     website_url: None,
///     body: "base64_encoded_image".to_string(),
///     module: None,
///     images: None,
/// }).await?;
///
/// // Poll for the result
/// let result = solver.poll_task_result(&task_id, 120, 5).await?;
/// # Ok(())
/// # }
/// ```
#[async_trait::async_trait]
pub trait CaptchaSolver: Send + Sync {
    /// Create a new captcha solving task.
    ///
    /// # Arguments
    /// * `task` - The task type and parameters
    ///
    /// # Returns
    /// The task ID as a `String` that can be used to query the task result.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The API key is invalid
    /// - The account has insufficient balance
    /// - The task parameters are invalid
    /// - Network communication fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use cap_solvers::{CapSolver, CaptchaSolver, TaskType};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let solver = CapSolver::new("YOUR_API_KEY");
    /// let task_id = solver.create_task(TaskType::ImageToText {
    ///     website_url: None,
    ///     body: "base64_encoded_image".to_string(),
    ///     module: None,
    ///     images: None,
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn create_task(&self, task: TaskType) -> Result<String>;

    /// Get the current result of a task.
    ///
    /// # Arguments
    /// * `task_id` - The task ID returned by [`create_task`](CaptchaSolver::create_task)
    ///
    /// # Returns
    /// The current task result, which may still be processing.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The task ID is not found
    /// - Network communication fails
    /// - The API returns an error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use cap_solvers::{CapSolver, CaptchaSolver, TaskStatus};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let solver = CapSolver::new("YOUR_API_KEY");
    /// # let task_id = "task-id";
    /// let result = solver.get_task_result(task_id).await?;
    /// match result.status {
    ///     TaskStatus::Ready => println!("Task complete!"),
    ///     TaskStatus::Processing => println!("Still processing..."),
    ///     TaskStatus::Failed => println!("Task failed"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    async fn get_task_result(&self, task_id: &str) -> Result<TaskResult>;

    /// Poll for task result with timeout.
    ///
    /// This is a convenience method that repeatedly calls [`get_task_result`](CaptchaSolver::get_task_result)
    /// until the task is complete or the timeout is reached.
    ///
    /// # Arguments
    /// * `task_id` - The task ID
    /// * `timeout_secs` - Maximum time to wait in seconds
    /// * `poll_interval_secs` - Time between polls in seconds
    ///
    /// # Returns
    /// The task result when ready or failed.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The timeout is reached before the task completes ([`Error::TaskTimeout`](crate::Error::TaskTimeout))
    /// - Any error from [`get_task_result`](CaptchaSolver::get_task_result) occurs
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use cap_solvers::{CapSolver, CaptchaSolver};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let solver = CapSolver::new("YOUR_API_KEY");
    /// # let task_id = "task-id";
    /// // Wait up to 120 seconds, checking every 5 seconds
    /// let result = solver.poll_task_result(task_id, 120, 5).await?;
    /// println!("Solution: {:?}", result.solution);
    /// # Ok(())
    /// # }
    /// ```
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

    /// Get account balance.
    ///
    /// # Returns
    /// Balance information including the current balance amount and currency.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The API key is invalid
    /// - Network communication fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use cap_solvers::{CapSolver, CaptchaSolver};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let solver = CapSolver::new("YOUR_API_KEY");
    /// let balance = solver.get_balance().await?;
    /// println!("Current balance: ${}", balance.balance);
    /// # Ok(())
    /// # }
    /// ```
    async fn get_balance(&self) -> Result<Balance>;
}
