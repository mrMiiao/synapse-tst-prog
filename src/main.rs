#![no_std]
#![no_main]

use core::arch::asm;

pub fn print_str(string: &str) {
    unsafe{
        asm!(
            "xor eax, eax",
            "mov ebx, {strp}",
            "int 80h",
            strp = in(reg) string.as_ptr()
        );
    }
}

#[no_mangle]
extern "C" fn _start() {
    print_str("Hello, World!\n");
}

#[panic_handler]
pub extern "C" fn panic(_code: &core::panic::PanicInfo) -> ! {
    print_str("Program panicked!\n");
    loop{}
}
