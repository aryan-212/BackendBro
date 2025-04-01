use super::command_line::PrintCommand;
use crate::{
    ai_functions::{
        ai_func_architect::print_project_scope, ai_func_managing::convert_user_input_to_goal,
    },
    models::{
        agents::agents_traits::ProjectScope,
        general::llm::{self, send_request},
    },
};
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fs;
pub const CODE_TEMPLATE_PATH: &str = "/home/aryan/BackendBro/web_template/src/code_template.rs";
pub const WEB_TEMPLATE_PATH: &str = "/home/aryan/BackendBro/web_template";
pub const EXEC_MAIN_PATH: &str = "/home/aryan/BackendBro/web_template/src/main.rs";
pub const API_SCHEMA_PATH: &str = "/home/aryan/BackendBro/schemas/api_schema.json";

// Get Code Template
pub fn read_code_to_template_contents() -> String {
    fs::read_to_string(CODE_TEMPLATE_PATH).expect("Something went wrong, failed to read template")
}
pub fn read_exec_main_contents() -> String {
    fs::read_to_string(EXEC_MAIN_PATH).expect("Something went wrong, failed to read template")
}
pub fn extract_code_block(input: String) -> Option<String> {
    // Find the first set of triple backticks (```).
    if let Some(start) = input.find("```") {
        // Find the second set of triple backticks (```), after the first one.
        if let Some(end) = input[start + 3..].find("```") {
            // Extract the content between the two sets of backticks.
            let code_block = &input[start + 3 + 4..start + 3 + end]; // Skip over "rust" or any language identifier
            return Some(code_block.to_string());
        }
    }

    // Return None if no code block is found
    None
}
pub fn save_backend_code(contents: &str) {
    let parsed_code = extract_code_block(contents.to_string());
    if let Some(code) = parsed_code {
        fs::write(EXEC_MAIN_PATH, code).expect("Failed to write main.rs");
    } else {
        eprintln!("Cannot parse the code");
    }
}

pub fn save_api_endpoints(api_endpoint: &str) {
    fs::write(API_SCHEMA_PATH, api_endpoint).expect("Couldn't write to file");
}

pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> String {
    let ai_function_str = ai_func(func_input);
    format!(
        "FUNCTION: {}\nINSTRUCTION: You are a function printer. \
        You ONLY print the results of functions. Nothing else. No commentary. \
        Here is the input to the function: {}. Print out what the function will return.",
        ai_function_str, func_input
    )
}

pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: fn(&str) -> &'static str,
) -> String {
    let extended_msg = extend_ai_function(function_pass, &msg_context);
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    match send_request(&extended_msg).await {
        Ok(response) => response,
        Err(_) => match send_request(&msg_context).await {
            Ok(fallback_response) => fallback_response,
            Err(e) => {
                eprintln!("Failed to call Gemini: {}", e);
                String::from("AI service unavailable")
            }
        },
    }
}
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: fn(&str) -> &'static str,
) -> T {
    let llm_response =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;
    println!("{}", llm_response);
    serde_json::from_str::<T>(&llm_response).expect("Failed to decode AI response from serde_json")
}

pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

pub fn read_code_template_contents() -> String {
    fs::read_to_string(CODE_TEMPLATE_PATH).expect("Failed to read code template")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::ai_func_architect::print_project_scope;
    use crate::ai_functions::ai_func_managing::convert_user_input_to_goal;
    use crate::models::agents::agents_traits::ProjectScope;

    #[test]
    fn tests_extending_ai_function() {
        let mssg = extend_ai_function(convert_user_input_to_goal, "dummy variable");
        println!("{}", mssg);
    }

    #[tokio::test]
    async fn tests_ai_task_request() {
        let ai_func_param = "display btc prices".to_string();
        let res = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;
        let res2: ProjectScope = ai_task_request_decoded::<ProjectScope>(
            res.clone(),
            "Solutions Architect",
            "Finding Project Scope",
            print_project_scope,
        )
        .await;
        dbg!(res2);
    }

    #[test]
    fn tests_convert_user_input_to_goal() {
        let user_input = "Build me a web site for making stock price API requests";
        let result = convert_user_input_to_goal(user_input);
        println!("{}", result);
    }
}
