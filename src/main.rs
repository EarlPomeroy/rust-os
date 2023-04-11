#![no_std] // don't link to the Rust standard lib
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

// Function to call on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

//static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}
