# Cap-Solvers-Rust

A Rust library for interacting with captcha solving services.

## Features

- ✅ Support for multiple captcha solving providers:
  - Anticaptcha
  - CapSolver
  - CapMonster
  - 2Captcha
- ✅ Unified async/await API
- ✅ Support for various captcha types:
  - Image to Text
  - ReCaptcha v2 (Proxyless & with Proxy)
  - ReCaptcha v3 (Proxyless & with Proxy)
  - ReCaptcha v3 Enterprise (Proxyless & with Proxy)
  - hCaptcha (Proxyless & with Proxy)
  - FunCaptcha (Proxyless & with Proxy)
  - Custom task types
- ✅ Proxy support for providers that support it
- ✅ Balance checking
- ✅ Task submission and polling
- ✅ Comprehensive error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cap_solvers = { git = "https://github.com/c-reetz/Cap-Solvers-Rust" }
```

## Usage

### Anticaptcha Example

```rust
use cap_solvers::{Anticaptcha, CaptchaSolver, TaskType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solver = Anticaptcha::new("YOUR_API_KEY");
    
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
    
    // Create a ReCaptcha V2 task (Proxyless)
    let task_id = solver.create_task(TaskType::ReCaptchaV2Proxyless {
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
    
    // Create an hCaptcha task (Proxyless)
    let task_id = solver.create_task(TaskType::HCaptchaProxyless {
        website_url: "https://example.com".to_string(),
        website_key: "site-key-here".to_string(),
    }).await?;
    
    // Poll for result
    let result = solver.poll_task_result(&task_id, 180, 5).await?;
    println!("Solution: {:?}", result.solution);
    
    Ok(())
}
```

### Proxy Support Example

```rust
use cap_solvers::{CapSolver, CaptchaSolver, ProxyConfig, TaskType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solver = CapSolver::new("YOUR_API_KEY");
    
    // Configure proxy
    let proxy = ProxyConfig {
        proxy_type: "http".to_string(),
        proxy_address: "proxy.example.com".to_string(),
        proxy_port: 8080,
        proxy_login: Some("username".to_string()),
        proxy_password: Some("password".to_string()),
    };
    
    // Create a ReCaptcha V3 task with proxy
    let task_id = solver.create_task(TaskType::ReCaptchaV3 {
        website_url: "https://example.com".to_string(),
        website_key: "site-key".to_string(),
        page_action: "login".to_string(),
        min_score: Some(0.7),
        proxy,
    }).await?;
    
    let result = solver.poll_task_result(&task_id, 180, 5).await?;
    println!("Solution: {:?}", result.solution);
    
    Ok(())
}
```

## Supported Task Types

- **ImageToText**: Basic image captcha solving
- **ReCaptchaV2Proxyless**: Google ReCaptcha v2 solving (without proxy)
- **ReCaptchaV2**: Google ReCaptcha v2 solving (with proxy)
- **ReCaptchaV3Proxyless**: Google ReCaptcha v3 solving (without proxy)
- **ReCaptchaV3**: Google ReCaptcha v3 solving (with proxy)
- **ReCaptchaV3EnterpriseProxyless**: Google ReCaptcha v3 Enterprise (without proxy)
- **ReCaptchaV3Enterprise**: Google ReCaptcha v3 Enterprise (with proxy)
- **HCaptchaProxyless**: hCaptcha solving (without proxy)
- **HCaptcha**: hCaptcha solving (with proxy)
- **FunCaptchaProxyless**: FunCaptcha (Arkose Labs) solving (without proxy)
- **FunCaptcha**: FunCaptcha (Arkose Labs) solving (with proxy)
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
# Anticaptcha
ANTICAPTCHA_API_KEY=your_key cargo run --example anticaptcha_example

# CapSolver
CAPSOLVER_API_KEY=your_key cargo run --example capsolver_example

# CapMonster
CAPMONSTER_API_KEY=your_key cargo run --example capmonster_example

# 2Captcha
TWOCAPTCHA_API_KEY=your_key cargo run --example twocaptcha_example

# Enterprise and Proxy Support
CAPSOLVER_API_KEY=your_key cargo run --example enterprise_and_proxy_example
```

## License
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
