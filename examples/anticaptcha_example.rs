//! Example of using Anticaptcha API

use cap_solvers::{Anticaptcha, CaptchaSolver, TaskType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = std::env::var("ANTICAPTCHA_API_KEY")
        .expect("ANTICAPTCHA_API_KEY environment variable not set");

    let solver = Anticaptcha::new(api_key);

    // Get balance
    println!("Fetching balance...");
    let balance = solver.get_balance().await?;
    println!(
        "Balance: ${:.2} {}",
        balance.balance,
        balance.currency.unwrap_or_default()
    );

    // Create an image-to-text task
    println!("\nCreating task...");
    let task = TaskType::ImageToText {
        body: "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==".to_string(),
    };

    let task_id = solver.create_task(task).await?;
    println!("Task created with ID: {}", task_id);

    // Poll for result (timeout: 120 seconds, poll interval: 5 seconds)
    println!("\nPolling for result...");
    let result = solver.poll_task_result(&task_id, 120, 5).await?;

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
