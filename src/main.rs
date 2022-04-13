#![no_std]
#![no_main]

extern "C" fn _main() {
    unsafe{core::arch::asm!("mov 0 $ax");}
}

#[panic_handler]
pub extern "C" fn panic(_code: &core::panic::PanicInfo) -> ! {
    loop{ }
}