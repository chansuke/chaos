#![feature(panic_implementation)]
#![no_std]
#![no_main]
#[macro_use]
extern crate lazy_static;
extern crate spin;

extern crate bootloader_precompiled;
extern crate volatile;

use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() {
    println!("Hello World{}", "!");

    loop {}
}

#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
