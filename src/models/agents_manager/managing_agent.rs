use crate::ai_functions::ai_func_managing::convert_user_input_to_goal;
use crate::helpers::general::ai_task_request;
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_architect::AgentSolutionArchitect;
use crate::models::agents::agent_backend::AgentBackendDeveloper;
use crate::models::agents::agent_traits::{FactSheet, SpecialFunctions};
use crate::models::general::llm::Message;

#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    fact_sheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
}

impl ManagingAgent {
    pub async fn new(user_request: String) -> Result<Self, Box<dyn std::error::Error>> {
        let attributes: BasicAgent = BasicAgent {
            objective: "Manages agents who are building a excelent website for the user."
                .to_string(),
            position: "Project Manager".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        let project_description: String = ai_task_request(
            user_request,
            &attributes.position,
            get_function_string!(convert_user_input_to_goal),
            convert_user_input_to_goal,
        )
        .await;
        let agents: Vec<Box<dyn SpecialFunctions>> = Vec::new();

        let fact_sheet: FactSheet = FactSheet {
            project_description: project_description,
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        return Ok(Self {
            attributes,
            fact_sheet,
            agents,
        });
    }

    fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.agents.push(agent);
    }

    fn create_agents(&mut self) {
        self.add_agent(Box::new(AgentSolutionArchitect::new()));
        self.add_agent(Box::new(AgentBackendDeveloper::new()));
        // TODO: Add more agents
    }

    pub async fn execute_project(&mut self) {
        self.create_agents();
        for agent in &mut self.agents {
            let result_agent: Result<(), Box<dyn std::error::Error>> =
                agent.execute(&mut self.fact_sheet).await;
            match result_agent {
                Ok(_) => (),
                Err(e) => panic!(
                    "Error detected during executing agent {}: {}",
                    agent.get_attributes_from_agent().position,
                    e
                ),
            }

            let agent_info = agent.get_attributes_from_agent();
            dbg!(agent_info);
        }
    }
}

#[cfg(test)]
mod tests {
    use core::borrow;

    use super::*;

    #[tokio::test]
    async fn tests_managing_agent() {
        let user_input = "Ik wil graag een full-stack webserver die todo kaarten bij houd. Ook moet ik dit per gebruiker kunnen doen. Ik wil graag het weer op elk kaartje zichtbaar hebben!".to_string();
        let mut agent: ManagingAgent = ManagingAgent::new(user_input)
            .await
            .expect("Failed to create Project Manager!");

        agent.execute_project().await;
        dbg!(&agent.fact_sheet);
        assert!(agent.fact_sheet.project_scope.is_some());
        assert!(agent.fact_sheet.external_urls.is_some());
    }
}
