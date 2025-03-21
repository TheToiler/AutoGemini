use crate::ai_functions::ai_func_architect::{print_project_scope, print_site_urls};
use crate::helpers::command_line::PrintCommand;
use crate::helpers::general::{ai_task_request_decoded, check_status_code};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};

// use crossterm::cursor::position;
use reqwest::Client;
use std::time::Duration;

// Solutions architect
#[derive(Debug)]
pub struct AgentSolutionArchitect {
    attributes: BasicAgent,
}

impl AgentSolutionArchitect {
    pub fn new() -> Self {
        let attributes: BasicAgent = BasicAgent {
            objective: "Gathers information and design solutions for website development"
                .to_string(),
            position: "Solutions architect".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };
        return Self { attributes };
    }

    async fn retrieve_project_scope(&mut self, factsheet: &mut FactSheet) -> ProjectScope {
        let msg_context: String = format!("{}", factsheet.project_description);
        let ai_response: ProjectScope = ai_task_request_decoded::<ProjectScope>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_project_scope),
            print_project_scope,
        )
        .await;

        factsheet.project_scope = Some(ai_response);
        self.attributes.update_state(AgentState::Finished);

        return ai_response;
    }

    async fn retrieve_determine_external_urls(
        &mut self,
        factsheet: &mut FactSheet,
        msg_context: String,
    ) {
        let ai_response: Vec<String> = ai_task_request_decoded::<Vec<String>>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_site_urls),
            print_site_urls,
        )
        .await;

        factsheet.external_urls = Some(ai_response);
        self.attributes.state = AgentState::UnitTesting;
    }
}

#[async_trait::async_trait]
impl SpecialFunctions for AgentSolutionArchitect {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        return &self.attributes;
    }

    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // ! ! ! WARNING: Be carefull of infinite loops ! ! !
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    let project_scope = self.retrieve_project_scope(factsheet).await;
                    if project_scope.is_external_urls_required {
                        self.retrieve_determine_external_urls(
                            factsheet,
                            factsheet.project_description.clone(),
                        )
                        .await;
                        self.attributes.state = AgentState::UnitTesting;
                    }
                }
                AgentState::UnitTesting => {
                    let mut exclude_urls: Vec<String> = vec![];
                    let client: Client = Client::builder()
                        .timeout(Duration::from_secs(5))
                        .build()
                        .unwrap();

                    // Find faulty URL's
                    let urls: &Vec<String> = factsheet
                        .external_urls
                        .as_ref()
                        .expect("No url object on factsheet");
                    for url in urls {
                        let endpoint_str: String = format!("Testing URL Endpoint: {}", url);
                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_ref(),
                            endpoint_str.as_str(),
                        );

                        // Perform url test
                        match check_status_code(&client, url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    exclude_urls.push(url.clone());
                                }
                            }
                            Err(e) => println!("Error checking url {}: {}", url, e),
                        };
                    }

                    // Exclude faulty urls
                    // TODO: Write a re-request for the model to redo the URL better.
                    if !exclude_urls.is_empty() {
                        let new_urls: Vec<String> = factsheet
                            .external_urls
                            .as_ref()
                            .unwrap()
                            .iter()
                            .filter(|url| !exclude_urls.contains(&url))
                            .cloned()
                            .collect();
                        factsheet.external_urls = Some(new_urls);
                    }

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
    async fn tests_solutions_architect() {
        let mut agent: AgentSolutionArchitect = AgentSolutionArchitect::new();

        let mut fact_sheet = FactSheet {
            project_description: "Build a full stack website with user login and logout that shows latest Forex prices".to_string(),
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,

        };

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
