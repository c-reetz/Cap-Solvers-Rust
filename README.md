# Cap-Solvers-Rust

A Rust library for interacting with captcha solving services.

## Features

- ✅ Support for multiple captcha solving providers:
  - CapSolver
  - CapMonster
  - 2Captcha
- ✅ Unified async/await API
- ✅ Support for various captcha types:
  - Image to Text
  - ReCaptcha v2
  - ReCaptcha v3
  - hCaptcha
  - FunCaptcha
  - Custom task types
- ✅ Balance checking
- ✅ Task submission and polling
- ✅ Comprehensive error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cap_solvers = "0.1.0"
```

## Usage

### CapSolver Example

```rust
use cap_solvers::{CapSolver, CaptchaSolver, TaskType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solver = CapSolver::new("YOUR_API_KEY");
    
    // Get balance
    let balance = solver.get_balance().await?;
    println!("Balance: ${}", balance.balance);
    
    // Create a task
    let task_id = solver.create_task(TaskType::ImageToText {
        body: "base64_encoded_image".to_string(),
    }).await?;
    
    // Poll for result (timeout: 120s, interval: 5s)
    let result = solver.poll_task_result(&task_id, 120, 5).await?;
    println!("Solution: {:?}", result.solution);
    
    Ok(())
}
```

### CapMonster Example

```rust
use cap_solvers::{CapMonster, CaptchaSolver, TaskType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solver = CapMonster::new("YOUR_API_KEY");
    
    // Get balance
    let balance = solver.get_balance().await?;
    println!("Balance: ${}", balance.balance);
    
    // Create a ReCaptcha V2 task
    let task_id = solver.create_task(TaskType::ReCaptchaV2 {
        website_url: "https://example.com".to_string(),
        website_key: "6Le-wvkSAAAAAPBMRTvw0Q4Muexq9bi0DJwx_mJ-".to_string(),
        is_invisible: Some(false),
    }).await?;
    
    // Poll for result
    let result = solver.poll_task_result(&task_id, 180, 5).await?;
    println!("Solution: {:?}", result.solution);
    
    Ok(())
}
```

### 2Captcha Example

```rust
use cap_solvers::{TwoCaptcha, CaptchaSolver, TaskType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solver = TwoCaptcha::new("YOUR_API_KEY");
    
    // Get balance
    let balance = solver.get_balance().await?;
    println!("Balance: ${}", balance.balance);
    
    // Create an hCaptcha task
    let task_id = solver.create_task(TaskType::HCaptcha {
        website_url: "https://example.com".to_string(),
        website_key: "site-key-here".to_string(),
    }).await?;
    
    // Poll for result
    let result = solver.poll_task_result(&task_id, 180, 5).await?;
    println!("Solution: {:?}", result.solution);
    
    Ok(())
}
```

## Supported Task Types

- **ImageToText**: Basic image captcha solving
- **ReCaptchaV2**: Google ReCaptcha v2 solving
- **ReCaptchaV3**: Google ReCaptcha v3 solving
- **HCaptcha**: hCaptcha solving
- **FunCaptcha**: FunCaptcha (Arkose Labs) solving
- **Custom**: Custom task types with flexible parameters

## API Methods

All providers implement the `CaptchaSolver` trait with these methods:

- `create_task(task: TaskType) -> Result<String>`: Submit a captcha task
- `get_task_result(task_id: &str) -> Result<TaskResult>`: Get task status and result
- `poll_task_result(task_id: &str, timeout_secs: u64, poll_interval_secs: u64) -> Result<TaskResult>`: Poll until task is complete or timeout
- `get_balance() -> Result<Balance>`: Get account balance

## Examples

Run the examples with:

```bash
# CapSolver
CAPSOLVER_API_KEY=your_key cargo run --example capsolver_example

# CapMonster
CAPMONSTER_API_KEY=your_key cargo run --example capmonster_example

# 2Captcha
TWOCAPTCHA_API_KEY=your_key cargo run --example twocaptcha_example
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
