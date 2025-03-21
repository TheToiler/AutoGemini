#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {{ stringify!($func) }};
}

#[macro_use]
mod ai_functions;
mod apis;
mod helpers;
mod models;

use apis::call_request::call_gemini;
use helpers::command_line::get_user_reponse;
use models::general::llm::{Message, MessagePart, MessagePartText};

#[tokio::main]
async fn main() {
    let user_input: String = get_user_reponse("What webserver are we building today?");

    let gemini_prompt: Message = Message {
        contents: vec![MessagePart {
            parts: vec![MessagePartText {
                text: user_input.to_string(),
            }],
        }],
        generation_config: None,
    };

    let gemini_response: String = match call_gemini(&gemini_prompt).await {
        Ok(response) => response,
        Err(_) => call_gemini(&gemini_prompt).await.unwrap(),
    };
    println!("{}", gemini_response);
}
