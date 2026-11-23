mod context;

pub use context::TrapContext;

use core::arch::global_asm;

use riscv::register::{
    scause::{self, Exception, Trap},
    stval,
    stvec::{self, TrapMode},
};

use crate::batch::run_next_app;
use crate::syscall::syscall;

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }

    unsafe {
        stvec::write(
            __alltraps as *const () as usize,
            TrapMode::Direct,
        );
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let cause = scause::read();
    let value = stval::read();

    match cause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            // 跳过ecall指令
            cx.sepc += 4;

            cx.x[10] = syscall(
                cx.x[17],
                [cx.x[10], cx.x[11], cx.x[12]],
            ) as usize;
        }

        Trap::Exception(Exception::StoreFault)
        | Trap::Exception(Exception::StorePageFault) => {
            println!(
                "[kernel] StoreFault: sepc={:#x}, stval={:#x}",
                cx.sepc,
                value
            );

            run_next_app();
        }

        Trap::Exception(Exception::IllegalInstruction) => {
            println!(
                "[kernel] IllegalInstruction: sepc={:#x}",
                cx.sepc
            );

            run_next_app();
        }

        _ => {
            panic!(
                "Unsupported trap {:?}, stval={:#x}",
                cause.cause(),
                value
            );
        }
    }

    cx
}
