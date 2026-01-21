//! Example demonstrating ReCaptcha V3 Enterprise with proxy support

use cap_solvers::{CapSolver, CaptchaSolver, ProxyConfig, TaskType};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key =
        std::env::var("CAPSOLVER_API_KEY").expect("CAPSOLVER_API_KEY environment variable not set");

    let solver = CapSolver::new(api_key);

    // Get balance
    println!("Fetching balance...");
    let balance = solver.get_balance().await?;
    println!(
        "Balance: ${:.2} {}",
        balance.balance,
        balance.currency.unwrap_or_default()
    );

    // Example 1: ReCaptcha V3 Enterprise (Proxyless)
    println!("\n=== Example 1: ReCaptcha V3 Enterprise (Proxyless) ===");
    let task_proxyless = TaskType::ReCaptchaV3EnterpriseProxyless {
        website_url: "https://example.com".to_string(),
        website_key: "6Le-wvkSAAAAAPBMRTvw0Q4Muexq9bi0DJwx_mJ-".to_string(),
        page_action: "login".to_string(),
        min_score: Some(0.7),
        enterprise_payload: None,
    };

    println!("Creating ReCaptcha V3 Enterprise task (Proxyless)...");
    let task_id = solver.create_task(task_proxyless).await?;
    println!("Task created with ID: {}", task_id);

    // Example 2: ReCaptcha V3 Enterprise (with Proxy)
    println!("\n=== Example 2: ReCaptcha V3 Enterprise (with Proxy) ===");
    let proxy = ProxyConfig {
        proxy_type: "http".to_string(),
        proxy_address: "proxy.example.com".to_string(),
        proxy_port: 8080,
        proxy_login: Some("username".to_string()),
        proxy_password: Some("password".to_string()),
    };

    let mut enterprise_payload = HashMap::new();
    enterprise_payload.insert("s".to_string(), serde_json::json!("custom_data"));

    let task_with_proxy = TaskType::ReCaptchaV3Enterprise {
        website_url: "https://example.com".to_string(),
        website_key: "6Le-wvkSAAAAAPBMRTvw0Q4Muexq9bi0DJwx_mJ-".to_string(),
        page_action: "login".to_string(),
        min_score: Some(0.7),
        enterprise_payload: Some(enterprise_payload),
        proxy,
    };

    println!("Creating ReCaptcha V3 Enterprise task (with Proxy)...");
    let task_id_proxy = solver.create_task(task_with_proxy).await?;
    println!("Task created with ID: {}", task_id_proxy);

    // Example 3: HCaptcha with Proxy
    println!("\n=== Example 3: HCaptcha (with Proxy) ===");
    let hcaptcha_proxy = ProxyConfig {
        proxy_type: "socks5".to_string(),
        proxy_address: "proxy.example.com".to_string(),
        proxy_port: 1080,
        proxy_login: None,
        proxy_password: None,
    };

    let hcaptcha_task = TaskType::HCaptcha {
        website_url: "https://accounts.hcaptcha.com/demo".to_string(),
        website_key: "a5f74b19-9e45-40e0-b45d-47ff91b7a6c2".to_string(),
        proxy: hcaptcha_proxy,
    };

    println!("Creating HCaptcha task (with Proxy)...");
    let hcaptcha_task_id = solver.create_task(hcaptcha_task).await?;
    println!("Task created with ID: {}", hcaptcha_task_id);

    println!("\n=== All tasks submitted successfully! ===");
    println!("Note: These tasks would normally be polled for results using poll_task_result()");

    Ok(())
}
