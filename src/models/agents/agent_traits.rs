#![allow(unused)]
use crate::models::agent_basic::basic_agent::BasicAgent;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FactSheet {
    pub project_description: String,
    pub project_scope: Option<ProjectScope>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub api_endpoint_schema: Option<Vec<RouteObject>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct ProjectScope {
    pub is_crud_required: bool, // true if site needs CRUD functionality
    pub is_user_login_and_logout: bool, // true if site needs users to be able to log in and log out
    pub is_external_urls_required: bool, // true if site needs to fetch data from third part providers
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RouteObject {
    pub route: String,
    pub is_route_dynamic: String,
    pub method: String,
    pub request_body: serde_json::Value,
    pub response: serde_json::Value,
}



pub trait SpecialFunctions: Debug {
    // Used so that manager  van get attributes from agents
    fn get_attributes_from_agent(&self) -> &BasicAgent;
}

pub trait ExecuteFunction: SpecialFunctions {
    // Excute agent on the factsheet
    async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>>;
}
