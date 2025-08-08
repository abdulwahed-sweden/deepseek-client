use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Role { System, User, Assistant }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Model {
    DeepseekChat,      // "deepseek-chat" → V3
    DeepseekReasoner,  // "deepseek-reasoner" → R1
}

impl Model {
    pub fn as_str(&self) -> &'static str {
        match self {
            Model::DeepseekChat => "deepseek-chat",
            Model::DeepseekReasoner => "deepseek-reasoner",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest<'a> {
    pub model: &'a str,
    pub messages: &'a [Message],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: MessageOut,
}

#[derive(Debug, Deserialize)]
pub struct MessageOut {
    pub role: Role,
    pub content: Option<String>,
    // لاحقًا نضيف reasoning_content لو بدنا
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
