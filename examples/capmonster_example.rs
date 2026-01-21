//! Example of using CapMonster API

use cap_solvers::{CapMonster, CaptchaSolver, TaskType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = std::env::var("CAPMONSTER_API_KEY")
        .expect("CAPMONSTER_API_KEY environment variable not set");

    let solver = CapMonster::new(api_key);

    // Get balance
    println!("Fetching balance...");
    let balance = solver.get_balance().await?;
    println!(
        "Balance: ${:.2} {}",
        balance.balance,
        balance.currency.unwrap_or_default()
    );

    // Create a ReCaptcha V2 task
    println!("\nCreating ReCaptcha V2 task...");
    let task = TaskType::ReCaptchaV2 {
        website_url: "https://www.google.com/recaptcha/api2/demo".to_string(),
        website_key: "6Le-wvkSAAAAAPBMRTvw0Q4Muexq9bi0DJwx_mJ-".to_string(),
        is_invisible: Some(false),
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
    if let Some(cost) = result.cost {
        println!("Cost: ${:.6}", cost);
    }

    Ok(())
}
