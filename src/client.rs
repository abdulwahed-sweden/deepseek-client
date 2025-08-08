use crate::error::{DeepSeekError, Result};
use crate::models::*;
use dotenvy::var;
use reqwest::{Client, StatusCode};

const DEFAULT_BASE: &str = "https://api.deepseek.com";

pub struct DeepSeekClient {
    http: Client,
    base_url: String,
    api_key: String,
}

impl DeepSeekClient {
    pub fn from_env() -> Result<Self> {
        let api_key = var("DEEPSEEK_API_KEY").map_err(|_| DeepSeekError::MissingApiKey)?;
        let base_url = var("DEEPSEEK_API_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE.to_string());
        Ok(Self {
            http: Client::new(),
            base_url,
            api_key,
        })
    }

    pub async fn chat(
        &self,
        model: Model,
        messages: &[Message],
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Result<ChatCompletionResponse> {
        let url = format!("{}/chat/completions", self.base_url);
        let body = ChatCompletionRequest {
            model: model.as_str(),
            messages,
            temperature,
            max_tokens,
            stream: Some(false),
        };

        let res = self.http
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;

        if status.is_success() {
            Ok(serde_json::from_str::<ChatCompletionResponse>(&text)?)
        } else {
            Err(DeepSeekError::Api {
                status: status.as_u16(),
                message: text,
            })
        }
    }
}
