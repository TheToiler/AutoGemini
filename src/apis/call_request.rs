#![allow(unused)]

use crate::models::general::llm::{
    GeminiResponse, GenerationConfig, Message, MessagePart, MessagePartText,
};
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};

use std::env;

//Call large language model (i.e. Gemini)
pub async fn call_gemini(message: &Message) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    //Extract API key
    let gemini_api_key: String =
        env::var("GEMINI_API_KEY").expect("API-Key for Gemini not provided in the .env file!");
    let gemini_url_prefix: String = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key=".to_string();

    // Combine url_prefix and api_key in single string
    let gemini_url = format!("{}{}", gemini_url_prefix, gemini_api_key);

    // Create headers
    let mut gemini_headers: HeaderMap = HeaderMap::new();
    gemini_headers.insert(
        "Content-Type",
        HeaderValue::from_str("application/json")
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    let client = reqwest::Client::builder()
        .default_headers(gemini_headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    let response: GeminiResponse = client
        .post(gemini_url)
        .json(&message)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    let mut response_string: String = String::new();
    for candidate in response.candidates {
        for parts in candidate.content.parts {
            if let Some(text) = parts.text.as_deref() {
                response_string.push_str(text);
            }
        }
    }
    return Ok(response_string);
}

#[tokio::test]
async fn test_call_gemini() {
    let message = "Hallo, dit is een test. Kan je een kort antwoord geven?";

    let gemini_prompt: Message = Message {
        contents: vec![MessagePart {
            parts: vec![MessagePartText {
                text: message.to_string(),
            }],
        }],
        generation_config: Some(GenerationConfig {
            temperature: Some(0.7),
            max_output_tokens: Some(500),
        }),
    };

    let res = call_gemini(&gemini_prompt).await;
    println!("Test result: {:#?}", res);
    if let Ok(_res_str) = res {
        assert!(true);
    } else {
        assert!(false);
    }
}
