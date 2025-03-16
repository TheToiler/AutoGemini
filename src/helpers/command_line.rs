use crossterm::{
    ExecutableCommand,
    style::{Color, ResetColor, SetForegroundColor},
};
use std::io::{stdin, stdout};

// Get user request
pub fn get_user_reponse(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();
    let stdin: std::io::Stdin = stdin();

    // Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!();
    println!("{}", question);

    //Read the userinput
    let mut user_input: String = String::new();
    stdout.execute(ResetColor).unwrap();
    stdin
        .read_line(&mut user_input)
        .expect("Failed to read input from stdin!");
    return user_input.trim().to_string();
}
