#![allow(dead_code)]

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
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

    /// Bright bit for **Foreground only**
    DarkGray = 8,
    /// Bright bit for **Foreground only**
    LightBlue = 9,
    /// Bright bit for **Foreground only**
    LightGreen = 10,
    /// Bright bit for **Foreground only**
    LightCyan = 11,
    /// Bright bit for **Foreground only**
    LightRed = 12,
    /// Bright bit for **Foreground only**
    Pink = 13,
    /// Bright bit for **Foreground only**
    Yellow = 14,
    /// Bright bit for **Foreground only**
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

struct Screen {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
    color_code: ColorCode,
    column_position: usize,
}

impl Screen {
    fn new(color_code: ColorCode) -> Self {
        Self {
            color_code,
            column_position: 0,
            chars: [(); 25].map(|_| {
                [(); 80].map(|_| ScreenChar {
                    ascii_character: b' ',
                    color_code,
                })
            }),
        }
    }
}

impl Buffer {
    fn copy(&mut self, other: &Screen) {
        for (s_row, o_row) in self.chars.iter_mut().zip(other.chars.iter()) {
            for (s_col, &o_col) in s_row.iter_mut().zip(o_row.iter()) {
                s_col.write(o_col);
            }
        }
    }
}

pub struct Writer {
    buffer: &'static mut Buffer,
    screens: [Screen; 2],
    screen: usize,
}

impl Writer {
    fn new() -> Self {
        Writer {
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
            screens: [
                Screen::new(ColorCode::new(Color::White, Color::Black)),
                Screen::new(ColorCode::new(Color::White, Color::LightGray)),
            ],
            screen: 0,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\x1b' => (),
            b'\x08' => {
                // Backspace
                if self.screens[self.screen].column_position > 0 {
                    self.screens[self.screen].column_position -= 1;
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.screens[self.screen].column_position;

                self.screens[self.screen].chars[row][col] = ScreenChar {
                    ascii_character: b' ',
                    color_code: self.screens[self.screen].color_code,
                };
            }
            b'\0' => {
                // Clear screen
                for row in 0..BUFFER_HEIGHT {
                    self.clear_row(row)
                }
                self.screens[self.screen].column_position = 0;
            }
            byte @ 0x20..=0x7e => {
                // Printable ASCII
                if self.screens[self.screen].column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.screens[self.screen].column_position;
                self.screens[self.screen].chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code: self.screens[self.screen].color_code,
                };
                self.screens[self.screen].column_position += 1;
            }
            _ => {
                if self.screens[self.screen].column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.screens[self.screen].column_position;
                self.screens[self.screen].chars[row][col] = ScreenChar {
                    ascii_character: 0xfe,
                    color_code: self.screens[self.screen].color_code,
                };
                self.screens[self.screen].column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        self.buffer.copy(&self.screens[self.screen]);
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.screens[self.screen].chars[row - 1][col] =
                    self.screens[self.screen].chars[row][col];
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.screens[self.screen].column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.screens[self.screen].color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.screens[self.screen].chars[row][col] = blank;
        }
    }
}

impl Default for Writer {
    fn default() -> Self {
        Self::new()
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
    (FG: $fg:expr, BG: $bg:expr, SCREEN: $scr:expr, $($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*), $fg, $bg, $scr));
    (FG: $fg:expr, SCREEN: $scr:expr, $($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*), $fg, Color::Black, $scr));
    (BG: $bg:expr, SCREEN: $scr:expr, $($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*), Color::White, $bg, $scr));
    (FG: $fg:expr, BG: $bg:expr, $($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*), $fg, $bg, 0));
    (FG: $fg:expr, $($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*), $fg, Color::Black, 0));
    (BG: $bg:expr, $($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*), Color::White, $bg, 0));
    (SCREEN: $scr:expr, $($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*), Color::White, Color::Black, $scr));
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*), $crate::vga_buffer::Color::White, $crate::vga_buffer::Color::Black, 0));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    (FG: $fg:expr, BG: $bg:expr, SCREEN: $scr:expr, $($arg:tt)*) => ($crate::print!(FG: $fg, BG: $bg, SCREEN: $scr, "{}\n", format_args!($($arg)*)));
    (FG: $fg:expr, SCREEN: $scr:expr, $($arg:tt)*) => ($crate::print!(FG: $fg, SCREEN: $scr, "{}\n", format_args!($($arg)*)));
    (BG: $bg:expr, SCREEN: $scr:expr, $($arg:tt)*) => ($crate::print!(BG: $bg, SCREEN: $scr, "{}\n", format_args!($($arg)*)));
    (FG: $fg:expr, BG: $bg:expr, $($arg:tt)*) => ($crate::print!(FG: $fg, BG: $bg, "{}\n", format_args!($($arg)*)));
    (FG: $fg:expr, $($arg:tt)*) => ($crate::print!(FG: $fg, "{}\n", format_args!($($arg)*)));
    (BG: $bg:expr, $($arg:tt)*) => ($crate::print!(BG: $bg, "{}\n", format_args!($($arg)*)));
    (SCREEN: $scr:expr, $($arg:tt)*) => ($crate::print!(SCREEN: $scr, "{}\n", format_args!($($arg)*)));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments, fg: Color, bg: Color, screen: usize) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let mut writer = WRITER.lock();
    writer.screen = screen;
    writer.screens[screen].color_code = ColorCode::new(fg, bg);
    interrupts::without_interrupts(|| {
        writer.write_fmt(args).unwrap();
    });
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
            let screen_char = writer.screens[writer.screen].chars[BUFFER_HEIGHT - 2][i];
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
