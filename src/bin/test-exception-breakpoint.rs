#![feature(panic_implementation)]
#![feature(abi_x86_interrupt)]
#![no_std]

#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate rust_os;
#[macro_use]
extern crate lazy_static;
extern crate x86_64;

use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::panic::PanicInfo;
use rust_os::exit_qemu;

static BREAKPOINT_HANDLER_CALLED: AtomicUsize = AtomicUsize::new(0);

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    unsafe { exit_qemu(); }
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init_idt();

    x86_64::instructions::int3();

    match BREAKPOINT_HANDLER_CALLED.load(Ordering::SeqCst) {
        1 => serial_println!("ok"),
        0 => {
            serial_println!("failed");
            serial_println!("Breakpoint handler was not called");
        },
        other => {
            serial_println!("failed");
            serial_println!("Breakpoint handler was called {} times", other);
        }
    }

    unsafe { exit_qemu(); }
    loop {}
}

/// Create the Interrupt Descriptor Table
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Set the custom breakpoint exception handler
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: &mut ExceptionStackFrame) {
    // Increment the atomic counter each time the breakpoint handler is called
    BREAKPOINT_HANDLER_CALLED.fetch_add(1, Ordering::SeqCst);
}

