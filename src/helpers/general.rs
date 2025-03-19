use std::fmt::format;
use std::io::{BufReader, Read, Write};

use serde::de::DeserializeOwned;
use reqwest::{Client, Response};

use crate::apis::call_request::call_gemini;
use crate::helpers::command_line::PrintCommand;
use crate::models::general::llm::{GeminiResponse, Message, MessagePart, MessagePartText};

const TEMPLATE_CODE: &str = "/home/arnold/Documents/Projects/Udemy/AutoGippity/web_template/src/code_template.rs";
const TEMPLATE_OUTPUT: &str = "/home/arnold/Documents/Projects/Udemy/AutoGippity/web_template/src/main.rs";
const TEMPLATE_API_ENDPOINT: &str = "/home/arnold/Documents/Projects/Udemy/AutoGippity/web_template/schemas/api_schema.json.rs";


pub fn extend_ai_function(ai_funct: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_string = ai_funct(func_input);

    let msg: String = format!(
        "FUNCTION: {}
        INSTRUCTION: You are a function printer. You ONLY print the result of functions.
        Nothing else. No commentary. Here is the input to the function: {}.
        Print out what the function will return.",
        ai_function_string, func_input
    );

    let gemini_prompt: Message = Message {
        contents: vec![MessagePart {
            parts: vec![MessagePartText {
                text: msg.to_string(),
            }],
        }],
        generation_config: None,
    };
    return gemini_prompt;
}

// Performs call to Gemini
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    //Extend the ai function
    let extended_message: Message = extend_ai_function(function_pass, &msg_context);

    dbg!(&extended_message);
    //Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    //Get LLM response
    let llm_response_res: Result<String, Box<dyn std::error::Error + Send>> =
        call_gemini(&extended_message).await;

    // Return succes or try again
    match llm_response_res {
        Ok(reponse) => reponse,
        Err(_) => call_gemini(&extended_message)
            .await
            .expect("Failed twice to call gemini"),
    }
}

// Performs call to Gemini and decode the result
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let response_to_decode = ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;
    let decoded_response: T = serde_json::from_str(&response_to_decode).expect("Failed to decode AI response from serde_json");

    // Return decoded response
    return decoded_response;
}



// Perform check on returned api links from the ai model.
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    return Ok(response.status().as_u16());
}

//Get Code template
pub fn read_code_template_contents() -> String {
    return std::fs::read_to_string(TEMPLATE_CODE.to_string()).expect("Failed to read code_template file!");
}


// Save new backend code
pub fn save_code(file_contents: &String) {
    std::fs::write(TEMPLATE_OUTPUT, file_contents.as_str()).expect("Failed to write code file!");
}

// Save api endpoint file
pub fn save_api_endpoint(api_endpoints: &String) {
    std::fs::write(TEMPLATE_API_ENDPOINT, api_endpoints.as_str()).expect("Failed to write API Endpoints to file!");
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::ai_func_managing::convert_user_input_to_goal;

    #[test]
    fn test_extend_ai_function() {
        let func_input = "Build me a rose";
        let extended_message: Message = extend_ai_function(convert_user_input_to_goal, func_input);
        println!("Result from extend_ai_func: {:#?}", extended_message);
        assert!(
            extended_message.contents[0].parts[0]
                .text
                .contains(func_input)
        )
    }

    #[tokio::test]
    async fn test_ai_task_request() {
        let ai_func_param: String = "Build me a sebserver for making stock price api requests! I want users to be able to register and login.".to_string();
        let result = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;
        println!("{}", result);
        assert!(result.len() > 20);
    }

    #[tokio::test]
    async fn test_check_url() {
        let client = reqwest::Client::new();
        let result = check_status_code(&client, "https://swapi.dev/api/people/").await;
        match result {
            Ok(code) => println!("We konden de URL bereiken: {}", code),
            Err(e) => println!("Error is: {:?}", e)
        };

        assert!(false);
    }

}
