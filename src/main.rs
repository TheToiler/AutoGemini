mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_reponse;
use models::general::llm::{Message, MessagePart, MessagePartText};
use apis::call_request::call_gemini;

#[tokio::main]
async fn main() {
    let user_input: String = get_user_reponse("What webserver are we building today?");
    dbg!(&user_input);

    let message_part_text = MessagePartText {
        text: user_input
    };
    let message_part = MessagePart { parts: vec![message_part_text]};
    let message = Message {
        contents: vec![message_part]
    };

    let gemini_response = call_gemini(message).await;
    println!("{}", gemini_response);
}
