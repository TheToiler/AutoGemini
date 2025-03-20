use crossterm::{
    ExecutableCommand,
    style::{Color, ResetColor, SetForegroundColor},
};
use std::io::{stdin, stdout};

#[allow(unused)]
#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(self, agent_position: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = std::io::stdout();

        //Decide on the print color
        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        // Print the statement in a specific color
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_position);
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);
        stdout.execute(ResetColor).unwrap();
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_agent_message() {
        PrintCommand::AICall.print_agent_message("architect", "something went wrong!");
    }
}
