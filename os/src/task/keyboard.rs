use crate::print;
use crate::println;
use crate::vga_buffer::Color;
use alloc::string::String;
use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::stream::Stream;
use futures_util::stream::StreamExt;
use futures_util::task::AtomicWaker;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if queue.push(scancode).is_err() {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    }
}

impl Default for ScancodeStream {
    fn default() -> Self {
        Self::new()
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(cx.waker());
        match queue.pop() {
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}

static WAKER: AtomicWaker = AtomicWaker::new();

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

    let mut text = String::new();
    print!(FG: Color::LightGray, "demon@SmolOS:~/$ ");

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(DecodedKey::Unicode(character)) = keyboard.process_keyevent(key_event) {
                print!("{}", character);
                if character == '\n' {
                    execute(&text);
                    text.clear();
                    print!(FG: Color::LightGray, "demon@SmolOS:~/$ ");
                } else if character == '\x08' {  // Backspace
                    text.pop();
                } else {
                    text.push(character);
                }
            }
        }
    }
}

fn execute(command: &str) {
    match command {
        "clear" => println!("\0"),
        "shut-down" => println!("Shut down your computer using the power button, we haven't implemented that yet"),
        "sys-info" => {
            println!("OS: SmolOS");
        }
        "poop" => println!(FG: Color::Brown,"Someone just pooped ;-;"),
        _ => println!(FG: Color::LightRed, "Unknown command"),
    }
}
