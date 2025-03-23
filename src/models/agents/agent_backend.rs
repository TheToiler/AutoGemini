#![allow(unused)]
use crate::ai_functions::ai_func_backend::{
    print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    print_rest_api_endpoints,
};
use crate::helpers::command_line::{ confirm_safe_code, PrintCommand };
use crate::helpers::general::{
    ai_task_request, check_status_code, read_code_template_contents,
    read_code_template_output_contents, save_api_endpoint, save_backend_code, WEB_SERVER_PROJECT_PATH,
};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};

use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::f32::consts::E;
use std::fs;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time;

use super::agent_traits::RouteObject;

// Solutions architect
#[derive(Debug)]
pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_count: u8,
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let attributes: BasicAgent = BasicAgent {
            objective: "Develops backend code for webserver and json database".to_string(),
            position: "Backend developer".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };
        return Self {
            attributes,
            bug_errors: None,
            bug_count: 0,
        };
    }

    async fn call_initial_backend_code(&mut self, fact_sheet: &mut FactSheet) {
        //Read in the code template
        let code_template = read_code_template_contents();

        //Concatenate instruction
        let mut msg_context: String = format!(
            "CODE TEMPLATE: {}\n PROJECT DESCRIPTION: {}\n",
            code_template, fact_sheet.project_description
        );

        // Generate initial code
        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await;
        assert!(!ai_response.contains("```"), "Detected codeblocks in the result from call_initial_backend_code!");
        save_backend_code(&ai_response);
        fact_sheet.backend_code = Some(ai_response);
    }

    async fn call_improved_backend_code(&mut self, fact_sheet: &mut FactSheet) {
        //Concatenate instruction
        let mut msg_context: String = format!(
            "CODE TEMPLATE: {:?}\n PROJECT DESCRIPTION: {:?}\n",
            fact_sheet.backend_code, fact_sheet
        );

        // Generate initial code
        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improved_webserver_code),
            print_improved_webserver_code,
        )
        .await;
        assert!(!ai_response.contains("```"), "Detected codeblocks in the result from call_improved_backend_code!");
        save_backend_code(&ai_response);
        fact_sheet.backend_code = Some(ai_response);
    }

    async fn call_fix_code_bugs(&mut self, fact_sheet: &mut FactSheet) {
        //Concatenate instruction
        let mut msg_context: String = format!(
            "BROKEN_CODE: {:?}\n ERROR_BUGS: {:?}\n
        THIS FUNCTION ONLY CODE. JUST OUPUT THE CODE. DO NOT PUT CODE IN CODE BLOCKS!",
            fact_sheet.backend_code, self.bug_errors
        );

        // Generate initial code
        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_fixed_code),
            print_fixed_code,
        )
        .await;
        assert!(!ai_response.contains("```"), "Detected codeblocks in the result from call_fix_code_bugs!");
        save_backend_code(&ai_response);
        fact_sheet.backend_code = Some(ai_response);
    }

    async fn call_rest_api_endpoints(mut self, fact_sheet: &mut FactSheet) -> String {
        let backend_main_code: String = read_code_template_output_contents();

        //Concatenate instruction
        let mut msg_context: String = format!("CODE_INPUT: {}\n", backend_main_code,);

        // Generate initial code
        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
        .await;

        // Debugging for the returned API Endpoints
        // dbg!(ai_response);
        return ai_response;

        // let route_object: Vec<RouteObject> = match serde_json::from_str(&ai_response){
        //     Ok(route_object) => route_object,
        //     Err(e) => panic!("ERROR Deserializing JSON: {} with the following error: {}", ai_response, e)
        // };
        // save_api_endpoint(&ai_response);
        // fact_sheet.api_endpoint_schema = Some(route_object);
        // return String::from("");
    }
}

#[async_trait::async_trait]
impl SpecialFunctions for AgentBackendDeveloper {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        return &self.attributes;
    }

    async fn execute(
        &mut self,
        fact_sheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // ! ! ! WARNING: Be carefull of infinite loops ! ! !
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    // self.call_initial_backend_code(fact_sheet).await;
                    // Confirm done
                    self.attributes.state = AgentState::Working;
                    continue;
                }
                AgentState::Working => {
                    // if self.bug_count == 0 {
                    //     self.call_improved_backend_code(fact_sheet).await;
                    // } else {
                    //     self.call_fix_code_bugs(fact_sheet).await;
                    // }
                    self.attributes.state = AgentState::UnitTesting;
                    continue;
                },
                AgentState::UnitTesting => {
                    PrintCommand::UnitTest.print_agent_message(&self.attributes.position, "Backend code unittesting: Ensuring safe code.");
                    let response = confirm_safe_code();
                    if response == false {
                        PrintCommand::UnitTest.print_agent_message(&self.attributes.position, "As requested stopped further UnitTesting.");
                        self.attributes.state = AgentState::Finished;
                        break;
                    };

                    PrintCommand::UnitTest.print_agent_message(&self.attributes.position, "Backend code unittesting: Building project.");
                    
                    // Building the code
                    let output: std::process::Output = Command::new("cargo")
                        .arg("build")
                        .arg("--message-format=json")
                        .current_dir(WEB_SERVER_PROJECT_PATH)
                        .output()
                        .expect("Apparently cargo is not installed or not available in the path!");
                    
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let mut error_count = 0;
                    for line in stdout.lines() {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                            if json["reason"] == "compiler-message" {
                                if let Some("error") = json["message"]["level"].as_str() {
                                    error_count += 1;
                                    self.bug_errors = Some(format!("{}{}", self.bug_errors.as_deref().unwrap_or(""), json["message"]["rendered"].as_str().unwrap_or("").to_string()));
                                }
                            }
                        }
                    }
                
                    println!("\nTotal Errors: {}", error_count);
                    println!("Content bug_errors: {}", self.bug_errors.as_deref().unwrap_or(""));
                    // Confirm done
                    self.attributes.state = AgentState::Finished;
                }
                // Default to finished state
                _ => {
                    self.attributes.state = AgentState::Finished;
                }
            }
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_backend_developer() {
        let mut agent: AgentBackendDeveloper = AgentBackendDeveloper::new();

        let factsheet_str: &str = r#"
            {
                "project_description": "build a website that fetches and tracks fitness progress with timezone information",
                "project_scope": {
                "is_crud_required": true,
                "is_user_login_and_logout": true,
                "is_external_urls_required": true
                },
                "external_urls": [
                "http://worldtimeapi.org/api/timezone"
                ],
                "backend_code": null,
                "api_endpoint_schema": null
            }"#;

        let mut fact_sheet: FactSheet = serde_json::from_str(factsheet_str).unwrap();

        // let project_scope = agent.retrieve_project_scope(&mut fact_sheet).await;
        // agent.retrieve_determine_external_urls(&mut fact_sheet, "".to_string()).await;
        agent
            .execute(&mut fact_sheet)
            .await
            .expect("Failed to execute on the agent");
        assert!(fact_sheet.project_scope.is_some());
        assert!(fact_sheet.external_urls.is_some());
        dbg!(fact_sheet);
    }
}
