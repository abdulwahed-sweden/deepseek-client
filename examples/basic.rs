use deepseek_client::{DeepSeekClient, Model, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let client = DeepSeekClient::from_env()?;

    let resp = client.chat()
        .model(Model::DeepseekChat) // Use Model enum for model selection
        .system("You are a helpful assistant.")
        .user("Say hello in Swedish!")
        .temperature(0.7)
        .max_tokens(128)
        .send()
        .await?;

    println!("{}", resp.choices[0].message.content.as_deref().unwrap_or(""));
    Ok(())
}

