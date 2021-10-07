#![no_std]

use core::panic::PanicInfo;

fn main() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// https://os.phil-opp.com/freestanding-rust-binary/#disabling-unwinding
