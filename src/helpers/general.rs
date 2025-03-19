use reqwest::Response;

use crate::models::general::llm::{Message, MessagePart, MessagePartText, GeminiResponse};
use crate::helpers::command_line::PrintCommand;
use crate::apis::call_request::call_gemini;

pub fn extend_ai_function(ai_funct: fn(&str) -> &'static str, func_input: &str) -> Message {

    let ai_function_string = ai_funct(func_input);

    let msg: String = format!("FUNCTION: {}
        INSTRUCTION: You are a function printer. You ONLY print the result of functions.
        Nothing else. No commentary. Here is the input to the function: {}.
        Print out what the function will return.",
        ai_function_string, func_input);

    let gemini_prompt: Message = Message {
        contents: vec![
            MessagePart {
                parts: vec![
                    MessagePartText { text: msg.to_string() }
                ]
            }
        ],
        generation_config: None
    };
    return gemini_prompt;
}


// Performs call to Gemini
pub async fn ai_task_request(msg_context: String, agent_position: &str, agent_operation: &str, function_pass: for<'a> fn(&'a str) -> &'static str) -> String {

    //Extend the ai function
    let extended_message: Message = extend_ai_function(function_pass, &msg_context);

    dbg!(&extended_message);
    //Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    //Get LLM response
    let llm_response_res: Result<String, Box<dyn std::error::Error + Send>> = call_gemini(&extended_message).await;

    // Return succes or try again
    match llm_response_res {
        Ok(reponse) => reponse,
        Err(_) => call_gemini(&extended_message).await.expect("Failed twice to call gemini")
    }
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
        assert!(extended_message.contents[0].parts[0].text.contains(func_input))
    }

    #[tokio::test]
    async fn test_ai_task_request() {
        let ai_func_param: String = "Build me a sebserver for making stock price api requests! I want users to be able to register and login.".to_string();
        let result = ai_task_request(ai_func_param, "Managing Agent", "Defining user requirements", convert_user_input_to_goal).await;
        println!("{}", result);
        assert!(result.len() > 20);
    }
}