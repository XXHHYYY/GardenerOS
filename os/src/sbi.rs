use core::arch::asm;

const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_SHUTDOWN: usize = 8;

#[inline(always)]
fn sbi_call(
    which: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> usize {
    let mut ret = arg0;

    unsafe {
        asm!(
            "ecall",
            inlateout("a0") ret,
            in("a1") arg1,
            in("a2") arg2,
            in("a7") which,
        );
    }

    ret
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);

    loop {
        core::hint::spin_loop();
    }
}
