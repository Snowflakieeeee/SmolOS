use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec::Vec,
};
use futures_util::stream::StreamExt;
use os::{print, println, smol_script, task::keyboard::ScancodeStream, vga_buffer::Color};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

pub async fn handle_main() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

    let mut command = String::new();
    let mut type_mode = false;
    let mut name = "DefaultUser".to_string();
    let mut files = Vec::<File>::new();
    print!(FG: Color::LightGreen, "{}@SmolOS:~/$ ", name);

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
                                    if file.content.pop().is_none() {
                                        continue;
                                    };
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
                                execute(&mut command, &mut type_mode, &mut files, &mut name);
                                command.clear();
                                print!(FG: Color::LightGreen, "{}@SmolOS:~/$ ", name);
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

fn execute<'a>(command: &str, type_mode: &mut bool, files: &mut Vec<File>, user_name: &mut String) {
    match *command.split_whitespace().collect::<Vec<_>>() {
        [] => (),
        ["clear"] => println!("\0"),
        ["hi" | "hello"] => println!("hello :)"),
        ["shut-down"] => println!(
            "Shut down your computer using the power button, we haven't implemented that yet"
        ),
        ["customize", "name", name] => {
            *user_name = name.to_string();
        }
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
            println!("     discard");
        }
        ["type"] => {
            if let Some(File { content, .. }) = files.last() {
                *type_mode = true;
                println!(FG: Color::Black, BG: Color::LightGray, SCREEN: 1, "\0Press Esc to exit");
                print!(FG: Color::White, BG: Color::LightGray, SCREEN: 1, "{}", content)
            } else {
                println!("No file opened");
            }
        }
        ["new"] => {
            if files.last().map(|x| x.name.is_none()).unwrap_or(false) {
                println!("Current file not saved");
            } else {
                files.push(File::new());
            }
        }
        ["save", filename] => {
            if let Some(File { name, .. }) = files.last_mut() {
                *name = Some((*filename).to_owned());
            } else {
                println!("No file has been opened");
            }
        }
        ["open", filename] => {
            if let Some(idx) = files
                .iter()
                .position(|x| matches!(x.name, Some(ref s) if s == filename))
            {
                let len = files.len() - 1;
                files.swap(idx, len);
            } else {
                println!("No such file found");
            }
        }
        ["delete", filename] => {
            if let Some(idx) = files
                .iter()
                .position(|x| matches!(x.name, Some(ref s) if s == filename))
            {
                files.remove(idx);
            } else {
                println!("No such file found");
            }
        }
        ["discard"] => {
            if files.pop().is_none() {
                println!("No file has been opened");
            }
        }
        ["ls"] => {
            if files.is_empty() {
                println!("No files to show");
            } else {
                println!("Files:");
                for file in files {
                    if let Some(name) = &file.name {
                        println!("     {}", name);
                    } else {
                        println!("     (unsaved)");
                    }
                }
            }
        }
        ["what", "is", "cellulose?"] => {
            println!("Cellulose is a type of organic compound that is found in the soil of plants. It is a natural building block for the synthesis of many other compounds. It is a polymer of Glucose");
        }
        ["run"] => match files.last() {
            None => println!("No file on the stack"),
            Some(File { name, content }) => smol_script::run(
                (name.as_ref().map(|s| &**s).unwrap_or("(unsaved)")).to_string(),
                content,
            ),
        },
        [a, "+", b] => {
            if let (Ok(a), Ok(b)) = (a.parse::<i32>(), b.parse::<i32>()) {
                println!("{}", a + b);
            } else {
                println!("Invalid input");
            }
        }
        [a, "*", b] => {
            if let (Ok(a), Ok(b)) = (a.parse::<i32>(), b.parse::<i32>()) {
                println!("{}", a * b);
            } else {
                println!("Invalid input");
            }
        }
        [a, "-", b] => {
            if let (Ok(a), Ok(b)) = (a.parse::<i32>(), b.parse::<i32>()) {
                println!("{}", a - b);
            } else {
                println!("Invalid input");
            }
        }
        [a, "/", b] => {
            if let (Ok(a), Some(b)) = (a.parse::<i32>(), b.parse::<i32>().ok().filter(|&a| a != 0))
            {
                println!("{}", a / b);
            } else {
                println!("Invalid input");
            }
        }
        ["poop"] => println!(FG: Color::Brown, "Someone just pooped ;-;"),
        _ => println!(FG: Color::LightRed, "Unknown command: '{}'", command),
    };
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
