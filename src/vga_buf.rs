//use crate::Alignment::{Center, Right};

const BUF_ADDR: u32 = 0xb8000;
const BUF_HEIGHT: u32 = 25;
const BUF_WIDTH: u32 = 80;

pub struct AsciiChar {
    pub char_byte: u8,
    pub color_byte: u8
}

#[derive(PartialEq)]
pub enum Alignment {
    Left, 
    Right, 
    Center
}

pub enum Color {
    Black,      //0x0
    DarkBlue,   //0x1
    Green,      //0x2
    Blue,       //0x3
    Red,        //0x4
    Purple,     //0x5
    Brown,      //0x6
    LightGrey,  //0x7
    DarkGrey,   //0x8
    BrightBlue, //0x9
    LightGreen, //0xa
    LightBlue,  //0xb
    LightRed,   //0xc
    Pink,       //0xd
    Yellow,     //0xe
    White       //0xf
}

pub struct Screen {
    buffer: *mut u8,
    color: u8,
    bg_color: u8,
    align: Alignment,
    line_pointer: u32,
    char_count: u32
}
//0x42 - B

impl core::fmt::Write for Screen {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            self.print(s);
        }
        Ok(())
    }
}

impl Screen {
    pub fn new(color: Color, bg_color: Color, align: Alignment) -> Screen {
        return Screen{
            buffer: BUF_ADDR as *mut u8,
            color: match color {
                Color::Black => 0x0,
                Color::DarkBlue => 0x1,
                Color::Green => 0x2,
                Color::Blue => 0x3,
                Color::Red => 0x4,
                Color::Purple => 0x5,
                Color::Brown => 0x6,
                Color::LightGrey => 0x7,
                Color::DarkGrey => 0x8,
                Color::BrightBlue => 0x9,
                Color::LightGreen => 0xa,
                Color::LightBlue => 0xb,
                Color::LightRed => 0xc,
                Color::Pink => 0xd,
                Color::Yellow => 0xe,
                Color::White => 0xf
            },
            bg_color: match bg_color {
                Color::Black => 0x0,
                Color::DarkBlue => 0x1,
                Color::Green => 0x2,
                Color::Blue => 0x3,
                Color::Red => 0x4,
                Color::Purple => 0x5,
                Color::Brown => 0x6,
                Color::LightGrey => 0x7,
                Color::DarkGrey => 0x8,
                Color::BrightBlue => 0x9,
                Color::LightGreen => 0xa,
                Color::LightBlue => 0xb,
                Color::LightRed => 0xc,
                Color::Pink => 0xd,
                Color::Yellow => 0xe,
                Color::White => 0xf
            },
            align,
            line_pointer: 0,
            char_count: 0
        }
    }

    pub fn print_hello_world(&mut self) {
        let mut i = 0;
        for byte in "Hello world!".bytes() {
            self.write_char(i, AsciiChar{char_byte: byte, color_byte: (self.bg_color << 4) | self.color});
            i += 1;
        }
    }

    pub unsafe fn print(&mut self, s: &str) {
        for byte in s.bytes() {
            if byte == '\n' as u8 {
                self.new_line();
                continue;
            }
            let offset: u32 = self.calc_offset();
            self.write_char(offset, AsciiChar{char_byte: byte, color_byte: (self.bg_color << 4) | self.color});
            self.char_count += 1;
        }
    }

    pub fn calc_offset(&mut self) -> u32 {
        if self.align == Alignment::Right {
            self.one_pos_left_offset(self.line_pointer);
            return BUF_WIDTH - 1 + self.line_pointer * BUF_WIDTH;
        } else if self.align == Alignment::Center {
            if self.char_count != 0 && self.char_count % 2 == 1 {
                self.one_pos_left_offset(self.line_pointer);
            }
            return BUF_WIDTH / 2 - 1 + self.char_count / 2 + self.line_pointer * BUF_WIDTH;
        }
        return self.char_count + self.line_pointer * BUF_WIDTH;
    }

    fn new_line(&mut self) {
        if self.line_pointer + 1 == BUF_HEIGHT {
            self.lift_up();
        } else {
            self.line_pointer += 1;
        }
        self.char_count = 0;
    }

    fn lift_up(&mut self) {
        for i in 0..BUF_HEIGHT - 1 {
            for j in 0..BUF_WIDTH{
                self.write_char(i * BUF_WIDTH + j, self.read_char((i + 1) * BUF_WIDTH + j));
            }
        }
        for j in 0..BUF_WIDTH {
            self.write_char((BUF_HEIGHT - 1) * BUF_WIDTH + j, AsciiChar{char_byte: 0, color_byte: (self.bg_color << 4) | self.color});
        }
    }
    
    fn one_pos_left_offset(&mut self, line_number: u32) {
        for j in 0..BUF_WIDTH-1 {
            self.write_char(line_number * BUF_WIDTH + j, self.read_char(line_number * BUF_WIDTH + j + 1));
        }
    }

    pub fn write_char(&self, offset: u32, char: AsciiChar) {
        unsafe {
            *self.buffer.offset(offset as isize * 2) = char.char_byte;
            *self.buffer.offset(offset as isize * 2 + 1) = char.color_byte;
        }
    }

    pub fn read_char(&self, offset: u32) -> AsciiChar {
        unsafe {
            return AsciiChar{
                char_byte: *self.buffer.offset(offset as isize * 2),
                color_byte: *self.buffer.offset(offset as isize * 2 + 1)
            }
        }
    }
}
