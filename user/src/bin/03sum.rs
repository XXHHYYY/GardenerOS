#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let mut sum = 0usize;

    for number in 1..=100 {
        sum += number;
    }

    println!("Sum from 1 to 100 = {}", sum);

    assert_eq!(sum, 5050);

    println!("Test sum OK!");

    0
}
