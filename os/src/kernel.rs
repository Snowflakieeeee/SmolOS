use alloc::string::String;
use futures_util::stream::StreamExt;
use os::{print, println, task::keyboard::ScancodeStream, vga_buffer::Color};
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};

pub async fn handle_kernel() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

    let mut text = String::new();
    let mut type_mode = false;
    print!(FG: Color::LightGray, "demon@SmolOS:~/$ ");

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        if type_mode {
                            run_editor(character, &mut type_mode)
                        } else {
                            run_kernel(&mut text, character, &mut type_mode)
                        }
                    }
                    DecodedKey::RawKey(key) => {
                        if !type_mode {
                            run_kernel_key(&mut text, key, &mut type_mode)
                        }
                    }
                }
            }
        }
    }
}

fn execute(command: &str, type_mode: &mut bool) {
    match command {
        "clear" => println!("\0"),
        "shut-down" => println!(
            "Shut down your computer using the power button, we haven't implemented that yet"
        ),
        "os-info" => {
            println!("OS: SmolOS");
            println!("Made in Rust");
            println!("Made by: Bunch-of-cells, Catt & SnmLogic");
        }
        "type" => {
            *type_mode = true;
        }
        "what is cellulose?" => {
            println!("Cellulose is a type of organic compound that is found in the soil of plants. It is a natural building block for the synthesis of many other compounds. It is a polymer of Glucose");
        }
        "poop" => println!(FG: Color::Brown, "Someone just pooped ;-;"),
        _ => println!(FG: Color::LightRed, "Unknown command: '{}'", command),
    }
}

fn run_kernel(text: &mut String, character: char, type_mode: &mut bool) {
    print!("{}", character);
    if character == '\n' {
        execute(text, type_mode);
        text.clear();
        print!(FG: Color::LightGray, "demon@SmolOS:~/$ ");
        if *type_mode {
            print!(SCREEN: 1, "\x1b");
        }
    } else if character == '\x08' {
        // Backspace
        text.pop();
    } else {
        text.push(character);
    }
}

fn run_kernel_key(_: &mut String, _: KeyCode, _: &mut bool) {}

fn run_editor(character: char, type_mode: &mut bool) {
    if character == '\x1b' {
        *type_mode = false;
        print!("\x1b");
    } else {
        print!(BG: Color::LightGray, SCREEN: 1, "{}", character);
    }
}
