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

        let mut last_was_tab = false;
        let mut tab_prefix = None;

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
                        let prefix = buffer
                            .split_whitespace()
                            .last()
                            .unwrap_or("");

                        if !last_was_tab {
                            // FIRST TAB
                            let matches = autocomplete::all_matches(prefix);

                            if matches.len() == 1 {
                                // Only one match → commit immediately
                                let before = buffer
                                    .rsplit_once(prefix)
                                    .map(|(a, _)| a)
                                    .unwrap_or("");

                                buffer = format!("{before}{} ", matches[0]);
                                cursor = buffer.len();
                            } else if matches.len() > 1 {
                                // Multiple matches → remember prefix, do nothing
                                tab_prefix = Some(prefix.to_string());
                            }

                            last_was_tab = true;
                        } else {
                            // SECOND TAB
                            let prefix = tab_prefix
                                .as_deref()
                                .unwrap_or(prefix);
                            if prefix != "" {
                                let matches = autocomplete::all_matches(prefix);
                            
                                println!();
                                for m in &matches {
                                    print!("{m}\n");
                                }
                                println!();
                            }
                            // Redraw prompt without changing buffer
                            print!("$ {}", buffer);
                            io::stdout().flush().unwrap();

                            last_was_tab = false;
                            tab_prefix = None;
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
