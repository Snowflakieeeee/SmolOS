#![allow(dead_code)]

use core::cell::Cell;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: Cell::new(ColorCode::new(Color::White, Color::Black)),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: Cell<ColorCode>,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            8 => {
                // Backspace
                if self.column_position > 0 {
                    self.column_position -= 1;
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code.get();
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: b' ',
                    color_code,
                });
            }
            b'\0' => {
                // Clear screen
                for row in 0..BUFFER_HEIGHT {
                    self.clear_row(row)
                }
                self.column_position = 0;
            }
            byte @ 0x20..=0x7e => {
                // Printable ASCII
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code.get();
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
            _ => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code.get();
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: 0xfe,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code.get(),
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    (FG: $fg:expr, BG: $bg:expr, $($arg:tt)*) => ($crate::vga_buffer::_print_fg_bg(format_args!($($arg)*), $fg, $bg));
    (FG: $fg:expr, $($arg:tt)*) => ($crate::vga_buffer::_print_fg_bg(format_args!($($arg)*), $fg, Color::Black));
    (BG: $bg:expr, $($arg:tt)*) => ($crate::vga_buffer::_print_fg_bg(format_args!($($arg)*), Color::White, $bg));
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    (FG: $fg:expr, BG: $bg:expr, $($arg:tt)*) => ({$crate::print!(FG: $fg, BG: $bg, "{}", format_args!($($arg)*)); print!("\n")});
    (FG: $fg:expr, $($arg:tt)*) => ($crate::print!(FG: $fg, "{}\n", format_args!($($arg)*)));
    (BG: $bg:expr, $($arg:tt)*) => ($crate::print!(BG: $bg, "{}\n", format_args!($($arg)*)));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _print_fg_bg(args: fmt::Arguments, fg: Color, bg: Color) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let mut writer = WRITER.lock();
    let color_code = writer.color_code.get();
    writer.color_code.set(ColorCode::new(fg, bg));
    interrupts::without_interrupts(|| {
        writer.write_fmt(args).unwrap();
    });
    writer.color_code.set(color_code);
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}
