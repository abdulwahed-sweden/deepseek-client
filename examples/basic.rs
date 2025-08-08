use deepseek_client::{DeepSeekClient, Model, Message, Role, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let client = DeepSeekClient::from_env()?;

    let msgs = vec![
        Message { role: Role::System, content: "You are a helpful assistant.".into() },
        Message { role: Role::User, content: "Say hello in Arabic!".into() },
    ];

    let resp = client.chat(Model::DeepseekChat, &msgs, Some(0.7), Some(128)).await?;
    println!("{}", resp.choices[0].message.content.as_deref().unwrap_or(""));
    Ok(())
}
