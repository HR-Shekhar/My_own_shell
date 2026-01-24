pub fn change_directory(args: &[&str]){
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