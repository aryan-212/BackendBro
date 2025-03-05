use crossterm::{
    ExecutableCommand,
    style::{Color, ResetColor, SetForegroundColor},
};
use std::io::{stdin, stdout};
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
