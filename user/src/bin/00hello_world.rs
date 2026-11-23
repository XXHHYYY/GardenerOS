#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Hello, world!");

    unsafe {
        // 用户态执行特权指令
        asm!("sret");
    }

    0
}
