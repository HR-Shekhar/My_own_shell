pub mod cd;
pub mod echo;
pub mod type_cmd;
pub mod clear;

pub fn is_builtin(cmd: &str) -> bool {
    matches!(cmd, "echo" | "exit" | "type" | "pwd" | "cd" | "cls" | "clear")
}

pub fn run(cmd: &str, args: &[&str]) -> bool {
    match cmd {
        "echo" => { echo::echo(&args.join(" ")); true }
        "pwd" => {
            println!("{}", std::env::current_dir().unwrap().display());
            true
        }
        "cd" => { cd::change_directory(args); true }
        "type" => { type_cmd::type_cmd(args); true }
        "clear"=> {clear::reset_terminal(); true}
        "cls" => {clear::clear_screen(); true}
        "exit" => false,
        _ => true,
    }
}

