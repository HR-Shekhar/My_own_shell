const BUILTINS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];

pub fn complete(prefix: &str) -> Option<&'static str> {
    for cmd in BUILTINS {
        if cmd.starts_with(prefix) {
            return Some(cmd);
        }
    }
    None
}