//! Example of using 2Captcha API

use cap_solvers::{TwoCaptcha, CaptchaSolver, TaskType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = std::env::var("TWOCAPTCHA_API_KEY")
        .expect("TWOCAPTCHA_API_KEY environment variable not set");

    let solver = TwoCaptcha::new(api_key);

    // Get balance
    println!("Fetching balance...");
    let balance = solver.get_balance().await?;
    println!("Balance: ${:.2} {}", balance.balance, balance.currency.unwrap_or_default());

    // Create an hCaptcha task
    println!("\nCreating hCaptcha task...");
    let task = TaskType::HCaptcha {
        website_url: "https://accounts.hcaptcha.com/demo".to_string(),
        website_key: "a5f74b19-9e45-40e0-b45d-47ff91b7a6c2".to_string(),
    };

    let task_id = solver.create_task(task).await?;
    println!("Task created with ID: {}", task_id);

    // Poll for result (timeout: 180 seconds, poll interval: 5 seconds)
    println!("\nPolling for result...");
    let result = solver.poll_task_result(&task_id, 180, 5).await?;
    
    println!("\nTask result:");
    println!("Status: {:?}", result.status);
    if let Some(solution) = result.solution {
        println!("Solution: {:?}", solution);
    }
    if let Some(error) = result.error {
        println!("Error: {}", error);
    }

    Ok(())
}
