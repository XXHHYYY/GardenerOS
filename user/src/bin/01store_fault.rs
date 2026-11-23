#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Into Test store_fault...");
    println!("Kernel should kill this application!");

    unsafe {
        // 向地址0写入
        asm!("sb zero, 0(zero)");
    }

    0
}
