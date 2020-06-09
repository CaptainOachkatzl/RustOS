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

#[repr(transparent)]
struct VGABuffer
{
    symbols: [[VGASymbol; BUFFER_WIDTH]; BUFFER_HEIGHT]
}


struct VGAWriter
{
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut VGABuffer
}

impl VGAWriter 
{
    fn write_char(&mut self, character: u8)
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

                self.buffer.symbols[row][col] = VGASymbol { ascii_character: character, color_code: self.color_code };
                self.column_position += 1;
            }
        }
    }

    fn write_string(&mut self, string: &str)
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

    fn write_line(&mut self, string: &str)
    {
        self.write_string(string);
        self.new_line();
    }

    fn new_line(&mut self)
    {
        for row in 1..BUFFER_HEIGHT 
        {
            for col in 0..BUFFER_WIDTH 
            {
                let character = self.buffer.symbols[row][col];
                self.buffer.symbols[row - 1][col] = character;
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
 
    }

    fn clear_screen(&mut self)
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
            self.buffer.symbols[row][col] = blank;
        }
    }
}

pub fn print_vga_test()
{
    let mut writer = VGAWriter 
    {
        column_position: 0,
        color_code: ColorCode::new(Color::Red, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut VGABuffer) },
    };

    writer.clear_screen();
    writer.write_line("writing a string");
    writer.write_string("writing in new line");
}


pub fn print_vga(text: &[u8]) {

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in text.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}
