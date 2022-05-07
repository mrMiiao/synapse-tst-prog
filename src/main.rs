#![no_std]
#![feature(start)]

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

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    print_str("Hello, World!\n");
    0
}


#[panic_handler]
pub extern "C" fn panic(_code: &core::panic::PanicInfo) -> ! {
    print_str("Program panicked!\n");
    loop{}
}
