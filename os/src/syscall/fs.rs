const FD_STDOUT: usize = 1;

const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if fd != FD_STDOUT {
        return -1;
    }

    if len == 0 {
        return 0;
    }

    let start = buf as usize;

    let end = match start.checked_add(len) {
        Some(end) => end,
        None => return -1,
    };

    // 输出缓冲区应位于用户应用或用户栈中
    let in_app = start >= APP_BASE_ADDRESS
        && end <= APP_BASE_ADDRESS + APP_SIZE_LIMIT;

    let in_stack = crate::batch::is_user_stack_range(start, end);

    if !in_app && !in_stack {
        return -1;
    }

    let bytes = unsafe {
        core::slice::from_raw_parts(buf, len)
    };

    for &byte in bytes {
        crate::sbi::console_putchar(byte as usize);
    }

    len as isize
}
