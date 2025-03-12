use crossterm::{
    ExecutableCommand,
    style::{Color, ResetColor, SetForegroundColor},
};
use std::io::{stdin, stdout};
#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}
impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();
        // Decide on the print color
        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };
        // Print the agent statement in a specific color
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent :{}", agent_pos);
        // Reset Color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        stdout.execute(ResetColor).unwrap();
    }
}
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();
    //Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);
    // Reset color
    stdout.execute(ResetColor).unwrap();
    // Read user input
    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");
    user_response.trim().to_string()
}
