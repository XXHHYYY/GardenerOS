#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

const SYSCALL_EXIT: usize = 93;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut a0 = args[0];
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") a0,
            in("a1") args[1],
            in("a2") args[2],
            in("a7") id,
            options(nostack)
        );
    }
    a0 as isize
}

pub fn sys_exit(status: i32) -> isize {
    syscall(SYSCALL_EXIT, [status as usize, 0, 0])
}

#[no_mangle]
pub extern "C" fn _start() {
    sys_exit(9);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
