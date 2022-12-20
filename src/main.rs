#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buf;
//mod game_of_life;

use core::fmt::Write;
use core::panic::PanicInfo;
//use core::ptr::write;
//use crate::game_of_life::game_of_life;
use crate::vga_buf::{Alignment, Screen};
use crate::vga_buf::Color::{Black, LightGreen, White};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    let mut screen = Screen::new(LightGreen, Black, Alignment::Right);

    for i in 0..100 {
        let _ = write!(screen, "Number {}\n", i);
    }

    //screen.print_hello_world();

    loop {}
}
