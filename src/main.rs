#[allow(unused_imports)]
use std::io::{self, Write};
use pathsearch::find_executable_in_path;
use std::process::Command;

fn execute_program(cmd: &str, args: &[&str]) {
    match Command::new(cmd).args(args).status() {
        Ok(_) => {}
        Err(e) => println!("{}: {}", cmd, e),
    }
}

fn change_directory(args: &[&str]){
    if args.len() == 1 {
        let path = args[0];
        if path=="~" || path.trim() == ""{
           if let Ok(home) = std::env::var("HOME"){
                if let Err(e) = std::env::set_current_dir(home) {
                    println!("cd: {}", e);
                }
            }
        } else if let Err(_) = std::env::set_current_dir(path){
            println!("{}: No such file or directory",path);
        }        
    } else {
        println!("No such file or directory");
    }
}

// fn builtin_echo(argument: &str){

// }

// fn parse_args(input: &str) -> Vec<String> {
//     enum State {
//         Normal,
//         SingleQuote,
//         DoubleQuote,
//     }

//     let mut args = Vec::new();
//     let mut current = String::new();
//     let mut state = State::Normal;

//     for c in input.chars() {
//         match state {
//             State::Normal => match c {
//                 ' ' => {
//                     if !current.is_empty() {
//                         args.push(current.clone());
//                         current.clear();
//                     }
//                 }
//                 '\'' => state = State::SingleQuote,
//                 '"' => state = State::DoubleQuote,
//                 _ => current.push(c),
//             },

//             State::SingleQuote => {
//                 if c == '\'' {
//                     state = State::Normal;
//                 } else {
//                     current.push(c);
//                 }
//             }

//             State::DoubleQuote => {
//                 if c == '"' {
//                     state = State::Normal;
//                 } else {
//                     current.push(c);
//                 }
//             }
//         }
//     }

//     if !current.is_empty() {
//         args.push(current);
//     }

//     args
// }
 

fn main() {

    let built_ins = ["echo", "exit", "type", "pwd", "cd"];

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

        if built_ins.contains(&command){
            match command {
                "echo" => println!("{}",arguments),
                "exit" => break,
                "pwd" => println!("{}", std::env::current_dir().unwrap().display()),
                "type" => {
                    if let Some(arg) = args.get(0){
                        if built_ins.contains(&arg){
                            println!("{} is a shell builtin", arg)
                        } else if let Some(path) = find_executable_in_path(arg) {
                            println!("{} is {}", arg, path.display());
                        } else {
                            println!("{}: not found", arg);
                        }
                    }
                },
                "cd" => change_directory(&args),

                // Prints the "<command>: command not found" message
                _ => println!("{}: command not found", command)
            }

        } else if find_executable_in_path(command).is_some() {
            execute_program(command, &args);
        } else {
            println!("{}: command not found", command);
        }
    }

}
