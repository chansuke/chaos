#![feature(panic_implementation)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#[macro_use]
#[cfg(test)]
extern crate lazy_static;
#[cfg(test)]
extern crate array_init;
extern crate bootloader_precompiled;
extern crate spin;
extern crate std;
extern crate uart_16550;
extern crate volatile;
extern crate x86_64;

#[cfg_attr(test, allow(unused_imports))]
use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;

#[macro_use]
mod serial;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    serial_println!("Hello Host{}", "!");

    unsafe {
        exit_qemu();
    }

    loop {}
}

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}
