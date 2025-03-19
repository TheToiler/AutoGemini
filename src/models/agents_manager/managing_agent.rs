use crate::models::agent_basic::basic_agent::{ AgentState, BasicAgent };
use crate::models::agents::agent_traits::{ FactSheet, SpecialFunctions };


#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    fact_sheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
}

