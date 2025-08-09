use thiserror::Error;

pub type Result<T> = std::result::Result<T, DeepSeekError>;

#[derive(Debug, Error)]
pub enum DeepSeekError {
    // General network/HTTP errors (Timeout, DNS, TLS…)
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    // JSON serialization/deserialization errors
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    // Configuration errors
    #[error("missing API key (set DEEPSEEK_API_KEY)")]
    MissingApiKey,

    // Common API error categories
    #[error("unauthorized (401): invalid or missing API key")]
    Unauthorized,
    #[error("forbidden (403): access denied")]
    Forbidden,
    #[error("insufficient balance (402): please top up your DeepSeek credits")]
    InsufficientBalance,
    #[error("rate limited (429): please retry later")]
    RateLimited,
    #[error("server error ({status}): please retry later")]
    Server { status: u16 },

    // Generic API error with raw message
    #[error("api error {status}: {message}")]
    Api { status: u16, message: String },
}

// Optional struct for parsing DeepSeek's API error format
#[derive(Debug, serde::Deserialize)]
pub struct ApiErrorEnvelope {
    pub error: Option<ApiErrorBody>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ApiErrorBody {
    pub message: Option<String>,
    pub r#type: Option<String>,
    pub code: Option<String>,
}
