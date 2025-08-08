use thiserror::Error;

pub type Result<T> = std::result::Result<T, DeepSeekError>;

#[derive(Debug, Error)]
pub enum DeepSeekError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("missing API key (set DEEPSEEK_API_KEY)")]
    MissingApiKey,
    #[error("api error {status}: {message}")]
    Api { status: u16, message: String },
}
