pub fn is_builtin(cmd: &str) -> bool {
    matches!(cmd, "echo" | "exit" | "type" | "pwd" | "cd" | "cls" | "clear")
}

pub fn type_cmd(args: &[&str]) {
    if let Some(arg) = args.get(0) {
        if is_builtin(arg) {
            println!("{} is a shell builtin", arg)
        } else if let Some(path) = pathsearch::find_executable_in_path(arg) {
            println!("{} is {}", arg, path.display());
        } else {
            println!("{}: not found", arg);
        }
    }
}