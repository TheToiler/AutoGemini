use crate::models::general::llm::{ Message, GeminiResponse };
use dotenv::dotenv;
use reqwest::{Client, Method};
use serde_json::Value;

use std::{collections::HashMap, env};

//Call large language model (i.e. Gemini)
pub async fn call_gemini(message: Message) -> String {
    dotenv().ok();

    //Extract API key
    let gemini_api_key: String = env::var("GEMINI_API_KEY").expect("API-Key for Gemini not provided in the .env file!");
    let gemini_url_prefix: String = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key=".to_string();

    // Combine url_prefix and api_key in single string
    let gemini_url = format!("{}{}", gemini_url_prefix, gemini_api_key);

    let client = reqwest::Client::new();
    let response: reqwest::Response = client.post(gemini_url)
        .header("Content-Type", "apllication/json")
        .body(serde_json::to_string(&message).unwrap())
        .send()
        .await
        .unwrap();
    //dbg!(&response.text().await.unwrap());
    let v: GeminiResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    println!("{:#?}", v.candidates.);
    //let return_value = response.json::<String>().await.unwrap();
    //return return_value;
    "Response".to_string()
}



