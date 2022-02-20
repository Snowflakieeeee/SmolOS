use alloc::{borrow::ToOwned, string::String, vec::Vec};
use futures_util::stream::StreamExt;
use os::{print, println, task::keyboard::ScancodeStream, vga_buffer::Color};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

pub async fn handle_kernel() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

    let mut command = String::new();
    let mut type_mode = false;
    let mut files = Vec::<File>::new();
    print!(FG: Color::LightGray, "demon@SmolOS:~/$ ");

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        if type_mode {
                            if let Some(file) = files.last_mut() {
                                if character == '\x1b' {
                                    type_mode = false;
                                    print!("\x1b");
                                    continue;
                                }
                                if character == '\x08' {
                                    file.content.pop();
                                } else {
                                    file.content.push(character);
                                }
                                print!(BG: Color::LightGray, SCREEN: 1, "{}", character);
                            }
                        } else {
                            if character == '\x08' && command.pop().is_none() {
                                continue;
                            }
                            print!("{}", character);
                            if character == '\n' {
                                match &*command.split_whitespace().collect::<Vec<_>>() {
                                    [] => (),
                                    ["clear"] => println!("\0"),
                                    ["shut-down"] => println!(
                                        "Shut down your computer using the power button, we haven't implemented that yet"
                                    ),
                                    ["os-info"] => {
                                        println!("OS: SmolOS");
                                        println!("Made in Rust");
                                        println!("Made by: Bunch-of-cells, Catt & SnmLogic");
                                    }
                                    ["help"] => {
                                        println!("Available commands:");
                                        println!("     clear");
                                        println!("     shut-down");
                                        println!("     os-info");
                                        println!("     help");
                                        println!("     type");
                                        println!("     ls");
                                        println!("     save");
                                        println!("     open");
                                        println!("     delete");
                                        println!("     save");
                                    }
                                    ["type"] => {
                                        if let Some(File {content, ..}) = files.last() {
                                            type_mode = true;
                                            println!(FG: Color::Black, BG: Color::LightGray, SCREEN: 1, "\0Press Esc to exit");
                                            print!(FG: Color::White, BG: Color::LightGray, SCREEN: 1, "{}", content)
                                        } else {
                                            println!("No file opened");
                                        }
                                    }
                                    ["new"] => {
                                        if files.last().map(|x|x.name.is_none()).unwrap_or(false) {
                                            println!("Current file not saved");
                                        } else {
                                            files.push(File::new());
                                        }
                                    }
                                    ["save", filename] => {
                                        if let Some(File { name, ..}) = files.last_mut() {
                                            *name = Some((*filename).to_owned());
                                        } else {
                                            println!("No file has been opened");
                                        }
                                    }
                                    ["open", filename] => {
                                        if let Some(idx) = files.iter().position(|x| matches!(x.name, Some(ref s) if s == *filename)) {
                                            let len = files.len() - 1;
                                            files.swap(idx, len);
                                        } else {
                                            println!("No such file found");
                                        }
                                    }
                                    ["delete", filename] => {
                                        if let Some(idx) = files.iter().position(|x| matches!(x.name, Some(ref s) if s == *filename)) {
                                            files.remove(idx);
                                        } else {
                                            println!("No such file found");
                                        }
                                    }
                                    ["what", "is" ,"cellulose?"] => {
                                        println!("Cellulose is a type of organic compound that is found in the soil of plants. It is a natural building block for the synthesis of many other compounds. It is a polymer of Glucose");
                                    }
                                    ["poop"] => println!(FG: Color::Brown, "Someone just pooped ;-;"),
                                    ["ls"] => {
                                        println!("Available files:");
                                        for file in &files {
                                            if let Some(ref name) = file.name {
                                                println!("     {}", name);
                                            }
                                        }
                                    }
                                    _ => println!(FG: Color::LightRed, "Unknown command: '{}'", command),
                                };
                                command.clear();
                                print!(FG: Color::LightGray, "demon@SmolOS:~/$ ");
                                if type_mode {
                                    print!(SCREEN: 1, "\x1b");
                                }
                            } else if character != '\x08' {
                                command.push(character);
                            }
                        }
                    }
                    DecodedKey::RawKey(_) => {}
                }
            }
        }
    }
}

struct File {
    name: Option<String>,
    content: String,
}

impl File {
    fn new() -> File {
        File {
            name: None,
            content: String::new(),
        }
    }
}

impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}
