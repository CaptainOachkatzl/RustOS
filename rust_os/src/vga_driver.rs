#[allow(dead_code)]
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

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VGASymbol
{
    ascii_character: u8,
    color_code: ColorCode
}

use volatile::Volatile;

#[repr(transparent)]
struct VGABuffer
{
    symbols: [[Volatile<VGASymbol>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}


pub struct VGAWriter
{
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut VGABuffer
}

#[allow(dead_code)]
impl VGAWriter 
{
    pub fn write_char(&mut self, character: u8)
    {
        match character // switch for rust
        {
            b'\n' => self.new_line(),   // if character == \n do a new line
            character => 
            {
                if self.column_position >= BUFFER_WIDTH
                {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.symbols[row][col].write(VGASymbol { ascii_character: character, color_code: self.color_code });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, string: &str)
    {
        for byte in string.bytes()
        {
            match byte 
            {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_char(byte),
                // not part of printable ASCII range
                _ => self.write_char(0xfe)
            }
        }
    }

    fn new_line(&mut self)
    {
        for row in 1..BUFFER_HEIGHT 
        {
            for col in 0..BUFFER_WIDTH 
            {
                let character = self.buffer.symbols[row][col].read();
                self.buffer.symbols[row - 1][col].write(character);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    pub fn clear_screen(&mut self)
    {
        for row in 0..BUFFER_HEIGHT
        {
            self.clear_row(row);
        }
    }

    fn clear_row(&mut self, row: usize)
    {
        let blank = VGASymbol { ascii_character: b' ', color_code: self.color_code };
        for col in 0..BUFFER_WIDTH
        {
            self.buffer.symbols[row][col].write(blank);
        }
    }
}

use core::fmt;

impl fmt::Write for VGAWriter // basically an override for an interface that allows the usage of string formatting macros
{
    fn write_str(&mut self, s: &str) -> fmt::Result 
    {
        self.write_string(s);
        Ok(())
    }
}

use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref WRITER: Mutex<VGAWriter> = Mutex::new(VGAWriter {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut VGABuffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_driver::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
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

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
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
            let screen_char = writer.buffer.symbols[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}