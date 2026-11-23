#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    let start = sbss as *const () as usize;
    let end = ebss as *const () as usize;

    for address in start..end {
        unsafe {
            (address as *mut u8).write_volatile(0);
        }
    }
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
