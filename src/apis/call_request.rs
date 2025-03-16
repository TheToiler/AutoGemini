use crate::models::general::llm::{ Message, GeminiResponse };
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};

use std::env;

//Call large language model (i.e. Gemini)
pub async fn call_gemini(message: &Message) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    //Extract API key
    let gemini_api_key: String = env::var("GEMINI_API_KEY").expect("API-Key for Gemini not provided in the .env file!");
    let gemini_url_prefix: String = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key=".to_string();

    // Combine url_prefix and api_key in single string
    let gemini_url = format!("{}{}", gemini_url_prefix, gemini_api_key);

    // Create headers
    let mut gemini_headers: HeaderMap = HeaderMap::new();
    gemini_headers.insert("Content-Type", HeaderValue::from_str("application/json")
        .map_err(|e| -> Box<dyn std::error::Error + Send> {Box::new(e)} )?
    );


    let client = reqwest::Client::builder();
    let response: reqwest::Response = client
        .default_headers(gemini_headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> {Box::new(e)} )?
        .post(gemini_url)
        .body(serde_json::to_string(&message)
            .map_err(|e| -> Box<dyn std::error::Error + Send> {Box::new(e)} )?
        )
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> {Box::new(e)} )?;
    //dbg!(&response.text().await.unwrap());
    // let v: GeminiResponse = serde_json::from_str(&response.text().await
    //     .map_err(|e| -> Box<dyn std::error::Error + Send> {Box::new(e)} )?
    // )
    // .map_err(|e| -> Box<dyn std::error::Error + Send> {Box::new(e)} )?;
    let gemini_reponse: GeminiResponse = response.json().await
        .map_err(|e| -> Box<dyn std::error::Error + Send> {Box::new(e)} )?;
    // println!("{:#?}", v.candidates[0].content.parts[0].text);
    let mut response_string: String = String::new();
    for candidate in gemini_reponse.candidates {
        for parts in candidate.content.parts {
            if let Some(text) = parts.text {
                response_string.push_str(&text);
            }
        }
    }
    return Ok(response_string);
}



