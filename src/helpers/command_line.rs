use crossterm::{
    ExecutableCommand,
    style::{Color, ResetColor, SetForegroundColor},
};
use std::io::{Write, stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout = stdout();

        // Decide on the print color
        let statement_color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        // Print the agent position in green
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent {}: ", agent_pos);

        // Set statement color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        // Reset color
        stdout.execute(ResetColor).unwrap();
        // let mut user_response = String::new();
        // stdin()
        //     .read_line(&mut user_response)
        //     .expect("Failed to read response");
        //
        // user_response.trim().to_string()
    }
}
pub fn confirm_safe_code() -> bool {
    let mut stdout = stdout();
    loop {
        //Print the questin in specified color
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("");
        println!("WARNING: You are about to run code written entirely by AI.");
        println!("Review your code and see if you want to continue");
        stdout.execute(ResetColor).unwrap();
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] All Good");
        stdout.execute(SetForegroundColor(Color::Red)).unwrap();
        println!("Let's stop this project");
        let mut human_response: String = String::new();
        stdin()
            .read_line(&mut human_response)
            .expect("Failed to read");
        // Trim whitespace
        let human_response = human_response.trim().to_lowercase();
        match human_response.as_str() {
            "1" | "ok" | "y" => return true,
            "2" | "no" | "n" => return false,
            _ => {
                println!("Invalid Input. Please select '1' or '2' ");
            }
        }
    }
}
pub fn get_user_response(question: &str) -> String {
    let mut stdout = stdout();

    // Print the question in blue
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("{}", question);

    // Reset color
    stdout.execute(ResetColor).unwrap();

    // Read user input
    let mut user_response = String::new();
    print!("> "); // Prompt symbol
    stdout.flush().unwrap();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    user_response.trim().to_string()
}
mod tests {
    use super::*;
    #[test]
    fn tests_prints_agent_msg() {
        PrintCommand::AICall.print_agent_message("Managing Agent", "It's processing something");
    }
}
