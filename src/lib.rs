//! # Cap Solvers
//!
//! A Rust library for interacting with captcha solving services.
//!
//! This crate provides a unified interface for multiple captcha solving providers,
//! allowing you to easily integrate captcha solving into your Rust applications.
//!
//! ## Supported Providers
//!
//! - [Anticaptcha](https://anti-captcha.com/)
//! - [CapSolver](https://www.capsolver.com/)
//! - [CapMonster](https://capmonster.cloud/)
//! - [2Captcha](https://2captcha.com/)
//!
//! ## Supported Captcha Types
//!
//! - **Image to Text**: Basic image captcha recognition
//! - **ReCaptcha v2**: Google ReCaptcha v2 (with and without proxy)
//! - **ReCaptcha v3**: Google ReCaptcha v3 (with and without proxy)
//! - **ReCaptcha v3 Enterprise**: Google ReCaptcha v3 Enterprise (with and without proxy)
//! - **hCaptcha**: hCaptcha solving (with and without proxy)
//! - **FunCaptcha**: FunCaptcha/Arkose Labs solving (with and without proxy)
//! - **Custom**: Custom task types with flexible parameters
//!
//! ## Features
//!
//! - 🔄 **Unified API**: All providers implement the [`CaptchaSolver`] trait
//! - ⚡ **Async/Await**: Built on tokio for efficient async operations
//! - 🔒 **Type-safe**: Leverages Rust's type system for correctness
//! - 🌐 **Proxy Support**: Configure proxies for captcha tasks that require them
//! - 💰 **Balance Checking**: Query account balance across all providers
//! - 🔁 **Automatic Polling**: Built-in polling with configurable timeouts
//! - 🛡️ **Rustls**: Uses rustls-tls for better cross-platform compatibility
//!
//! ## Quick Start
//!
//! ```no_run
//! use cap_solvers::{CapSolver, CaptchaSolver, TaskType};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a solver instance
//!     let solver = CapSolver::new("YOUR_API_KEY");
//!     
//!     // Check your balance
//!     let balance = solver.get_balance().await?;
//!     println!("Balance: ${}", balance.balance);
//!     
//!     // Submit a captcha task
//!     let task_id = solver.create_task(TaskType::ImageToText {
//!         website_url: None,
//!         body: "base64_encoded_image".to_string(),
//!         module: None,
//!         images: None,
//!     }).await?;
//!     
//!     // Poll for the result (timeout: 120s, interval: 5s)
//!     let result = solver.poll_task_result(&task_id, 120, 5).await?;
//!     
//!     if let Some(solution) = result.solution {
//!         println!("Solution: {:?}", solution);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Using Different Providers
//!
//! All providers implement the same [`CaptchaSolver`] trait, so you can easily
//! switch between them:
//!
//! ```no_run
//! use cap_solvers::{Anticaptcha, CapMonster, TwoCaptcha, CaptchaSolver};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Use any provider with the same API
//! let anticaptcha = Anticaptcha::new("YOUR_API_KEY");
//! let capmonster = CapMonster::new("YOUR_API_KEY");
//! let twocaptcha = TwoCaptcha::new("YOUR_API_KEY");
//!
//! // All implement the same methods
//! let balance1 = anticaptcha.get_balance().await?;
//! let balance2 = capmonster.get_balance().await?;
//! let balance3 = twocaptcha.get_balance().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Proxy Configuration
//!
//! For captcha types that require proxy support:
//!
//! ```no_run
//! use cap_solvers::{CapSolver, CaptchaSolver, ProxyConfig, TaskType};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let solver = CapSolver::new("YOUR_API_KEY");
//!
//! let proxy = ProxyConfig {
//!     proxy_type: "http".to_string(),
//!     proxy_address: "proxy.example.com".to_string(),
//!     proxy_port: 8080,
//!     proxy_login: Some("username".to_string()),
//!     proxy_password: Some("password".to_string()),
//! };
//!
//! let task_id = solver.create_task(TaskType::ReCaptchaV2 {
//!     website_url: "https://example.com".to_string(),
//!     website_key: "6Le-wvkSAAAAAPBMRTvw0Q4Muexq9bi0DJwx_mJ-".to_string(),
//!     is_invisible: Some(false),
//!     proxy,
//! }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling
//!
//! The crate uses a comprehensive [`Error`] type that covers all possible error
//! conditions:
//!
//! ```no_run
//! use cap_solvers::{CapSolver, CaptchaSolver, Error, TaskType};
//!
//! # async fn example() {
//! let solver = CapSolver::new("YOUR_API_KEY");
//!
//! match solver.create_task(TaskType::ImageToText {
//!     website_url: None,
//!     body: "image_data".to_string(),
//!     module: None,
//!     images: None,
//! }).await {
//!     Ok(task_id) => println!("Task created: {}", task_id),
//!     Err(Error::InvalidApiKey) => eprintln!("Invalid API key"),
//!     Err(Error::InsufficientBalance) => eprintln!("Insufficient balance"),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! # }
//! ```

pub mod error;
pub mod providers;
pub mod types;

pub use error::{Error, Result};
pub use providers::{
    anticaptcha::Anticaptcha, capmonster::CapMonster, capsolver::CapSolver, twocaptcha::TwoCaptcha,
};
pub use types::{Balance, CaptchaSolver, ProxyConfig, TaskResult, TaskStatus, TaskType};
