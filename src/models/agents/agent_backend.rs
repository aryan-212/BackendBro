use crate::ai_functions::ai_func_backend::{
    print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    print_rest_api_endpoints,
};
use crate::helpers::general::extract_code_block;
use crate::helpers::general::{
    WEB_TEMPLATE_PATH, check_status_code, read_code_template_contents, read_exec_main_contents,
    save_api_endpoints,
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
        // dbg!(&ai_response);
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
        let ai_response = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
        .await;

        extract_code_block(ai_response)
            .expect("Can't extract code")
            .trim()
            .to_string()
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
                    // Determine if build bug_errors
                    if build_backend_server.status.success() {
                        self.bug_count = 0;
                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(),
                            "Backend Code Unit Testing: Test server build successfull..",
                        );
                        self.attributes.state = AgentState::Finished
                    } else {
                        let error_arr: Vec<u8> = build_backend_server.stderr;
                        let error_str = String::from_utf8(error_arr).unwrap();
                        self.bug_count += 1;
                        self.bug_errors = Some(error_str);

                        if self.bug_count > 3 {
                            PrintCommand::Issue.print_agent_message(
                                self.attributes.position.as_str(),
                                "Backend Code Unit Testing: Too many bugs found in Code",
                            );
                            panic!("Error: Too many bugs")
                        }
                        // Pass back for rework
                        self.attributes.state = AgentState::Working;
                        continue;
                    }
                    // fixed the issue
                    /*
                      Extract and Test
                      Rest API Endpoints
                    */

                    // Extract API Endpoints
                    let api_endpoints_str: String = self.call_extract_rest_api_endpoints().await;
                    println!("-----GALTI SHAAYAD YAHAN HUA HAI !!!!-------");
                    // println!("{}", api_endpoints_str);
                    // Convert API Endpoints into Values
                    let api_endpoints: Vec<RouteObject> =
                        serde_json::from_str(api_endpoints_str.as_str())
                            .expect("Failed to decode API Endpoints");

                    // Define endpoints to check
                    let check_endpoints: Vec<RouteObject> = api_endpoints
                        .iter()
                        .filter(|&route_object| {
                            route_object.method == "get" && route_object.is_route_dynamic == "false"
                        })
                        .cloned()
                        .collect();

                    // Store API Endpoints
                    factsheet.api_endpoint_schema = Some(check_endpoints.clone());

                    // Run backend application
                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend Code Unit Testing: Starting web server...",
                    );

                    // Execute running server
                    let mut run_backend_server: std::process::Child = Command::new("cargo")
                        .arg("run")
                        .current_dir(WEB_TEMPLATE_PATH)
                        .spawn()
                        .expect("Failed to run backend application");

                    // Let user know testing on server will take place soon
                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend Code Unit Testing: Launching tests on server in 5 seconds...",
                    );

                    let seconds_sleep: Duration = Duration::from_secs(5);
                    time::sleep(seconds_sleep).await;

                    // Check status code
                    for endpoint in check_endpoints {
                        // Confirm url testing
                        let testing_msg: String =
                            format!("Testing endpoint '{}'...", endpoint.route);
                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(),
                            testing_msg.as_str(),
                        );

                        // Create client with timout
                        let client: Client = Client::builder()
                            .timeout(Duration::from_secs(5))
                            .build()
                            .unwrap();

                        // Test url
                        let url: String = format!("http://localhost:8080{}", endpoint.route);
                        match check_status_code(&client, &url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    let err_msg: String = format!(
                                        "WARNING: Failed to call backend url endpoint {}",
                                        endpoint.route
                                    );
                                    PrintCommand::Issue.print_agent_message(
                                        self.attributes.position.as_str(),
                                        err_msg.as_str(),
                                    );
                                }
                            }
                            Err(e) => {
                                // kill $(lsof -t -i:8080)
                                run_backend_server
                                    .kill()
                                    .expect("Failed to kill backend web server");
                                let err_msg: String = format!("Error checking backend {}", e);
                                PrintCommand::Issue.print_agent_message(
                                    self.attributes.position.as_str(),
                                    err_msg.as_str(),
                                );
                            }
                        }
                    }

                    save_api_endpoints(&api_endpoints_str);

                    PrintCommand::UnitTest.print_agent_message(
                        self.attributes.position.as_str(),
                        "Backend testing complete...",
                    );

                    run_backend_server
                        .kill()
                        .expect("Failed to kill backend web server on completion");

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
