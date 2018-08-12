#![feature(panic_implementation)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate chaos;

use chaos::exit_qemu;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
  panic!();
}

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
  serial_println!("ok");

  unsafe {
    exit_qemu();
  }
  loop {}
}
