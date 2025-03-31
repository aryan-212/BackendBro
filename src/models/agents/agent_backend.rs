use crate::ai_functions::ai_func_backend::{
    print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    print_rest_api_endpoints,
};
use crate::helpers::general::{
    WEB_TEMPLATE_PATH, check_status_code, read_code_template_contents, read_exec_main_contents,
};
use crate::{ai_task_request_decoded, save_backend_code};

use crate::helpers::command_line::{PrintCommand, confirm_safe_code};
use crate::helpers::general::ai_task_request;
use crate::models::agents::agents_traits::{FactSheet, RouteObject, SpecialFunctions};
use crate::models::agents_basic::basic_agent::{AgentState, BasicAgent};

use async_trait::async_trait;
use crossterm::cursor::position;
use crossterm::style::Print;
use reqwest::Client;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time;

use super::agents_traits::ProjectScope;
#[derive(Debug)]
pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_count: u8,
}
impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let attributes: BasicAgent = BasicAgent {
            objective: "Manage agents who are building an excellent website for the uesr"
                .to_string(),
            position: "Backend Developer".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };
        Self {
            attributes,
            bug_errors: None,
            bug_count: 0,
        }
    }
    async fn call_initial_backend_code(&mut self, factsheet: &mut FactSheet) {
        let code_template_str: String = read_code_template_contents();
        //Concatenate Instructions
        let mut msg_context: String = format!(
            "CODE TEMPLATE : {} \n  PROJECT_DESCRIPTION:{} \n",
            code_template_str, factsheet.project_description
        );
        let ai_response = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await;
        dbg!(&ai_response);
        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }
    async fn call_improved_backend_code(&mut self, factsheet: &mut FactSheet) {
        let code_template_str: String = read_code_template_contents();
        //Concatenate Instructions
        let mut msg_context: String = format!(
            "CODE TEMPLATE : {:?} \n  PROJECT_DESCRIPTION:{:?} \n",
            code_template_str, factsheet.project_description
        );
        let ai_response = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await;
        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }
    async fn call_fix_code_bugs(&mut self, factsheet: &mut FactSheet) {
        let code_template_str: String = read_code_template_contents();
        //Concatenate Instructions
        let mut msg_context: String = format!(
            " BROKEN CODE :{:?} \n  PROJECT_DESCRIPTION:{:?} \n
                THIS FUNCTION ONLY OUTCPUTS CODE. JUST OUTPUT THE CODE",
            factsheet.backend_code, self.bug_errors
        );
        let ai_response = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_fixed_code),
            print_fixed_code,
        )
        .await;
        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }
    async fn call_extract_rest_api_endpoints(&self) -> String {
        let backend_code = read_exec_main_contents();
        // Structuring msg_context
        let msg_context = format!("CODE_INPUT: {}", backend_code);
        let ai_response = ai_task_request_decoded::<String>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
        .await;

        ai_response
    }
}
#[async_trait]
impl SpecialFunctions for AgentBackendDeveloper {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    self.call_initial_backend_code(factsheet).await;
                    self.attributes.state = AgentState::Working;
                }
                AgentState::Working => {
                    if self.bug_count == 0 {
                        self.call_improved_backend_code(factsheet).await;
                    } else {
                        self.call_fix_code_bugs(factsheet).await;
                    }
                    self.attributes.state = AgentState::UnitTesting;
                }
                AgentState::UnitTesting => {
                    // Here you might want to perform some unit testing actions
                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend Code Unit Testing : Requesting User Input",
                    );
                    let is_safe_code = confirm_safe_code();
                    if !is_safe_code {
                        panic!("Better go work on some AI alignment");
                    }
                    // Build and test code
                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend Code Unit Testing: Building Agent",
                    );
                    let build_backend_server = Command::new("cargo")
                        .arg("build")
                        .current_dir(WEB_TEMPLATE_PATH)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                        .expect("Failed to run Backend Application");
                    self.attributes.state = AgentState::Finished;
                }
                _ => {
                    // Handle any other states if needed
                }
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn tests_writing_backend_code() {
        let mut agent = AgentBackendDeveloper::new();
        let factsheet_str = r#"{
    "project_description": "build a website that streams video.\n",
    "project_scope": {
        "is_crud_required": true,
        "is_user_login_and_logout": false,
        "is_external_urls_required": true
    },
    "external_urls": [
        "https://api.dailymotion.com/videos?fields=id,title,thumbnail_url"
    ],
    "backend_code": null,
    "api_endpoint_schema": null
}"#;
        let mut factsheet: FactSheet = serde_json::from_str(factsheet_str).unwrap();
        dbg!(&factsheet);
        agent
            .execute(&mut factsheet)
            .await
            .expect("Failed to execute Backend Developer code");
    }
}
