use deepseek_client::{DeepSeekClient, Model, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let client = DeepSeekClient::from_env()?;

    // Use the builder + match for clear error messages
    match client
        .chat()
        .model(Model::DeepseekChat) // Use Model enum for model selection
        .system("You are a helpful assistant.")
        .user("Say hello in Swedish!")
        .temperature(0.7)
        .max_tokens(128)
        .send()
        .await
    {
        Ok(resp) => {
            println!("{}", resp.choices[0].message.content.as_deref().unwrap_or(""));
        }
        Err(deepseek_client::DeepSeekError::Unauthorized) => {
            eprintln!("Auth failed: check DEEPSEEK_API_KEY.")
        }
        Err(deepseek_client::DeepSeekError::Forbidden) => {
            eprintln!("Access denied (403).")
        }
        Err(deepseek_client::DeepSeekError::InsufficientBalance) => {
            eprintln!("Insufficient balance (402).")
        }
        Err(deepseek_client::DeepSeekError::RateLimited) => {
            eprintln!("Rate limited (429). Please retry later.")
        }
        Err(deepseek_client::DeepSeekError::Server { status }) => {
            eprintln!("Server error ({status}). Try again later.")
        }
        Err(e) => eprintln!("Error: {e}"),
    }

    Ok(())
}
