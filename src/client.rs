use crate::error::{DeepSeekError, Result};
use crate::models::*;
use dotenvy::var;
use reqwest::Client;

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

    pub fn chat(&self) -> ChatRequestBuilder {
        ChatRequestBuilder {
            client: self,
            model: Model::DeepseekChat,
            messages: Vec::new(),
            temperature: None,
            max_tokens: None,
            stream: Some(false),
        }
    }

    async fn send_chat_request(
        &self,
        model: Model,
        messages: &[Message],
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        stream: Option<bool>,
    ) -> Result<ChatCompletionResponse> {
        let url = format!("{}/chat/completions", self.base_url);
        let body = ChatCompletionRequest {
            model: model.as_str(),
            messages,
            temperature,
            max_tokens,
            stream,
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
            if status.as_u16() == 402 && text.contains("Insufficient Balance") {
                return Err(DeepSeekError::InsufficientBalance);
            }
            Err(DeepSeekError::Api {
                status: status.as_u16(),
                message: text,
            })
        }
    }
}

pub struct ChatRequestBuilder<'a> {
    client: &'a DeepSeekClient,
    model: Model,
    messages: Vec<Message>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    stream: Option<bool>,
}

impl<'a> ChatRequestBuilder<'a> {
    pub fn model(mut self, model: Model) -> Self {
        self.model = model;
        self
    }

    pub fn system<S: Into<String>>(mut self, content: S) -> Self {
        self.messages.push(Message { role: Role::System, content: content.into() });
        self
    }

    pub fn user<S: Into<String>>(mut self, content: S) -> Self {
        self.messages.push(Message { role: Role::User, content: content.into() });
        self
    }

    pub fn assistant<S: Into<String>>(mut self, content: S) -> Self {
        self.messages.push(Message { role: Role::Assistant, content: content.into() });
        self
    }

    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn max_tokens(mut self, max: u32) -> Self {
        self.max_tokens = Some(max);
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub async fn send(self) -> Result<ChatCompletionResponse> {
        self.client
            .send_chat_request(
                self.model,
                &self.messages,
                self.temperature,
                self.max_tokens,
                self.stream,
            )
            .await
    }
}
