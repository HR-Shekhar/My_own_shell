use std::env;
use std::fs;

const BUILTINS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];


pub fn complete(prefix: &str) -> Option<String> {
        let executables = executable_in_path();

        for cmd in BUILTINS {
            if cmd.starts_with(prefix) {
                return Some(cmd.to_string());
            }
        }
        for exec in executables {
            if exec.starts_with(prefix) {
                return Some(exec);
            }
        }
        None
    }

fn executable_in_path() -> Vec<String> {
    let mut cmds = Vec::new();
    let  path = env::var("PATH").unwrap_or_default();
    for dir in env::split_paths(&path) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_file(){
                    continue;
                }
                // println!("{:#?}",path.file_name());
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    cmds.push(name.to_string());
                }
            }
        }
    }
    cmds
}