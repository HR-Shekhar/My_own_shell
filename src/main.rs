#[allow(unused_imports)]
use std::io::{self, Write};
use pathsearch::find_executable_in_path;
mod builtins;
use my_shell::execute_program;

fn main() {

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        // Captures the user's command in the "command" variable
        io::stdin().read_line(&mut input).unwrap();

        let input_parsed = match input.trim().split_once(' ') {
            Some(input_parsed) => input_parsed, // Command with arguments.
            None => (input.trim(), ""),  // Command with no arguments.
        };

        let command = input_parsed.0;
        let arguments = input_parsed.1;
        
        let args: Vec<&str> = arguments.split_whitespace().collect();

        if builtins::is_builtin(command) {
            let should_continue = builtins::run(command, &args);
            if !should_continue {
                break;
            }
        } else if find_executable_in_path(command).is_some() {
            execute_program(command, &args);
        } else {
            println!("{}: command not found", command);
        }
    }

}
