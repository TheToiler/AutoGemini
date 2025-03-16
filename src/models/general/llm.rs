use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub contents: Vec<MessagePart>
}

#[derive(Debug, Serialize, Clone)]
pub struct MessagePart {
    pub parts: Vec<MessagePartText>
}


#[derive(Debug, Serialize, Clone)]
pub struct MessagePartText {
    pub text: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeminiResponse {
    pub candidates: Vec<ResponseContent>,
    #[serde(rename = "usageMetadata")]
    pub usage_metadata: Option<UsageMetadata>, // Optional field
    #[serde(rename = "modelVersion")]
    pub model_version: Option<String> // Optional field
}

#[derive(Debug, Deserialize, Clone)]
pub struct UsageMetadata {
    #[serde(rename = "promptTokenCount")]
    pub prompt_token_count: Option<u32>,
    #[serde(rename = "candidatesTokenCount")]
    pub candidates_token_count: Option<u32>,
    #[serde(rename = "totalTokenCount")]
    pub total_token_count: Option<u32>,
    #[serde(rename = "promptTokensDetails")]
    pub prompt_tokens_details: Option<Vec<TokenDetails>>,
    #[serde(rename = "candidatesTokensDetails")]
    pub candidates_tokens_details: Option<Vec<TokenDetails>>
}

#[derive(Debug, Deserialize, Clone)]
pub struct TokenDetails {
    pub modality: Option<String>,
    #[serde(rename = "tokenCount")]
    pub token_count: Option<u32>
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResponseContent {
    pub content: ResponseContentParts,
    #[serde(rename = "finishReason")]
    pub finish_reason: Option<String>,
    pub avg_logprobs: Option<f64>
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResponseContentParts {
    pub parts: Vec<ResponseContentPartsText>,
    pub role: Option<String>
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResponseContentPartsText {
    pub text: Option<String>
}
