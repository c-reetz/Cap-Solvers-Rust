//! # Cap Solvers
//!
//! A Rust library for interacting with captcha solving services.
//!
//! This crate provides a unified interface for multiple captcha solving providers:
//! - CapSolver
//! - CapMonster
//! - 2Captcha
//!
//! ## Example
//!
//! ```no_run
//! use cap_solvers::{CapSolver, CaptchaSolver, TaskType};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let solver = CapSolver::new("YOUR_API_KEY");
//!     
//!     // Get balance
//!     let balance = solver.get_balance().await?;
//!     println!("Balance: ${}", balance.balance);
//!     
//!     // Submit a task (example with a simple task)
//!     let task_id = solver.create_task(TaskType::ImageToText {
//!         body: "base64_encoded_image".to_string(),
//!     }).await?;
//!     
//!     // Poll for result
//!     let result = solver.get_task_result(&task_id).await?;
//!     println!("Result: {:?}", result);
//!     
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod providers;
pub mod types;

pub use error::{Error, Result};
pub use providers::{capmonster::CapMonster, capsolver::CapSolver, twocaptcha::TwoCaptcha};
pub use types::{Balance, CaptchaSolver, ProxyConfig, TaskResult, TaskStatus, TaskType};
