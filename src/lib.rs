#![no_std]

extern crate bootloader_precompiled;
extern crate spin;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate uart_16550;
extern crate x86_64;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

pub mod serial;
pub mod vga_buffer;

pub unsafe fn exit_qemu() {
  use x86_64::instructions::port::Port;

  let mut port = Port::<u32>::new(0xf4);
  port.write(0);
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello World{}", "!");

  init_idt();

  // trigger a page fault
  unsafe {
    *(0xdeadbeef as *mut u64) = 42;
  };

  println!("It did not crash!");
  loop {}
}
