
#![no_std]
#![no_main]

use core::panic::PanicInfo;


/// The ! marker type is used to indicate that a function will never return.
/// See diverging functions in the Rust book for more details.
/// https://doc.rust-lang.org/1.30.0/book/first-edition/functions.html#diverging-functions
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//  Defines the entry point for the program.
// This function is called when the program starts.
// On a typical rust binary, the entry point is something called crt0,
// which is a C executable runtime that sets up the environment and calls the  _start function in rust,
// but in this case we are defining our own entry point as we do not want to rely on the OS or crt0.
#[no_mangle] 
// This disables name mangling, ensuring hthe rust compiler outputs a function with exactly the name that we declared it.
pub extern "C" fn _start() -> ! {
// The extern C is used to tell the compiler that it must use the C calling convention to call this function

    loop {}
}
