#![feature(panic_implementation)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#[macro_use]
#[cfg(test)]
extern crate lazy_static;
#[cfg(test)]
extern crate array_init;
extern crate spin;
extern crate std;

extern crate bootloader_precompiled;
extern crate volatile;

#[cfg_attr(test, allow(unused_imports))]
use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
