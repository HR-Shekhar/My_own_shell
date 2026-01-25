#[allow(unused_imports)]
use core::error;
use std::io::{self, Write};
use pathsearch::find_executable_in_path;
mod builtins;
use my_shell::execute_program;
mod autocomplete;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, read};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::{cursor::MoveToColumn, ExecutableCommand};

fn main() {

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();   //stores what user typed
        let mut cursor = 0;                  //stores current position of the cursor


        enable_raw_mode().unwrap();
        loop {
            if let Event::Key(key) = read().unwrap() {
                // IGNORE key releases and repeats (Windows sends duplicates)
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                
                match key.code {

                    // 1. FILTER GARBAGE INPUT (critical)
                    // Only allow printable ASCII + space
                    KeyCode::Char(c)
                        if (c.is_ascii_graphic() || c == ' ')  // arm works only if this statement is true
                            && !key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        buffer.insert(cursor, c);
                        cursor += 1;
                    },

                    KeyCode::Backspace => {
                        if cursor > 0 {
                            buffer.remove(cursor - 1);
                            cursor -= 1;
                        }
                    }

                    // 3. HANDLE ARROW KEYS (prevents escape-sequence corruption)
                    KeyCode::Left => {
                        if cursor > 0 {
                            cursor -= 1;
                        }
                    }

                    KeyCode::Right => {
                        if cursor < buffer.len() {
                            cursor += 1;
                        }
                    }

                    // 4. TAB AUTOCOMPLETE
                    KeyCode::Tab => {
                        if let Some(cmd) = autocomplete::complete(&buffer) {
                            buffer = format!("{} ", cmd);
                            cursor = buffer.len();
                        }
                    }

                    KeyCode::Enter => break,
                    _ => {}

                }
                print!("\r\x1b[2K$ {}", buffer);
                io::stdout().flush().unwrap();

                // 6. MANUAL CURSOR SYNC (this fixes "hheell" bug)
                io::stdout()
                    .execute(MoveToColumn((2 + cursor) as u16))
                    .unwrap();
            }
        }

        disable_raw_mode().unwrap();
        println!();

        let input_parsed = match buffer.trim().split_once(' ') {
            Some(input_parsed) => input_parsed, // Command with arguments.
            None => (buffer.trim(), ""),  // Command with no arguments.
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
