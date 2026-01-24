use std::process::Command;


pub fn execute_program(cmd: &str, args: &[&str]) {
    match Command::new(cmd).args(args).status() {
        Ok(_) => {}
        Err(e) => println!("{}: {}", cmd, e),
    }
}