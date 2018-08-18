use chaos::exit_qemu;
use core::sync::atomic::{AtomicUsize, Ordering};

static BREAKPOINT_HANDLER_CALLED: AtomicUsize = AtomicUsize::new(0);

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
  init_idt();

  x86_64::instructions::int3();

  match BREAKPOINT_HANDLER_CALLED.load(Ordering::SeqCst) {
    1 => serial_println!("ok"),
    0 => {
      serial_println!("failed");
      serial_println!("Breakpoint handler was not called.");
    }
    other => {
      serial_println!("failed");
      serial_println!("Breakpoint handler was called {} times", other);
    }
  }

  unsafe {
    exit_qemu();
  }
  loop {}
}

extern "x86-interrupt" fn breakpoint_handler(_: &mut ExceptionStackFrame) {
  BREAKPOINT_HANDLER_CALLED.fetch_add(1, Ordering::SeqCst);
}
