use core::arch::asm;
use core::cell::{RefCell, UnsafeCell};
use lazy_static::lazy_static;

use crate::trap::TrapContext;

const MAX_APP_NUM: usize = 16;

const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;

struct AppManager {
    inner: RefCell<AppManagerInner>,
}

struct AppManagerInner {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

// 本实验使用单hart执行
unsafe impl Sync for AppManager {}

impl AppManagerInner {
    fn print_app_info(&self) {
        println!("[kernel] num_app = {}", self.num_app);

        for index in 0..self.num_app {
            println!(
                "[kernel] app_{} [{:#x}, {:#x})",
                index,
                self.app_start[index],
                self.app_start[index + 1]
            );
        }
    }

    unsafe fn load_app(&self, app_id: usize) {
        let start = self.app_start[app_id];
        let end = self.app_start[app_id + 1];

        let app_size = end
            .checked_sub(start)
            .expect("Invalid application boundaries");

        assert!(
            app_size <= APP_SIZE_LIMIT,
            "Application is too large!"
        );

        println!("[kernel] Loading app_{}", app_id);

        // 清空应用运行区域
        for address in
            APP_BASE_ADDRESS..APP_BASE_ADDRESS + APP_SIZE_LIMIT
        {
            (address as *mut u8).write_volatile(0);
        }

        USER_STACK.clear();

        let source = core::slice::from_raw_parts(
            start as *const u8,
            app_size,
        );

        let destination = core::slice::from_raw_parts_mut(
            APP_BASE_ADDRESS as *mut u8,
            app_size,
        );

        destination.copy_from_slice(source);

        // 新代码写入后刷新指令缓存
        asm!("fence.i");
    }
}

lazy_static! {
    static ref APP_MANAGER: AppManager = AppManager {
        inner: RefCell::new({
            extern "C" {
                fn _num_app();
            }

            let num_app_ptr =
                _num_app as *const () as *const usize;

            let num_app = unsafe {
                num_app_ptr.read_volatile()
            };

            assert!(num_app <= MAX_APP_NUM);

            let mut app_start = [0usize; MAX_APP_NUM + 1];

            let raw = unsafe {
                core::slice::from_raw_parts(
                    num_app_ptr.add(1),
                    num_app + 1,
                )
            };

            app_start[..=num_app].copy_from_slice(raw);

            AppManagerInner {
                num_app,
                current_app: 0,
                app_start,
            }
        }),
    };
}

pub fn init() {
    APP_MANAGER.inner.borrow().print_app_info();
}

pub fn run_next_app() -> ! {
    // 离开作用域后释放借用
    let current_app = {
        let manager = APP_MANAGER.inner.borrow();

        if manager.current_app >= manager.num_app {
            println!("[kernel] All applications completed!");
            crate::sbi::shutdown();
        }

        let current_app = manager.current_app;

        unsafe {
            manager.load_app(current_app);
        }

        current_app
    };

    APP_MANAGER.inner.borrow_mut().current_app =
        current_app + 1;

    let context = TrapContext::app_init_context(
        APP_BASE_ADDRESS,
        USER_STACK.get_sp(),
    );

    let context_address =
        KERNEL_STACK.push_context(context);

    extern "C" {
        fn __restore(cx_addr: usize) -> !;
    }

    unsafe {
        __restore(context_address);
    }
}

#[repr(align(4096))]
struct KernelStack {
    data: UnsafeCell<[u8; KERNEL_STACK_SIZE]>,
}

#[repr(align(4096))]
struct UserStack {
    data: UnsafeCell<[u8; USER_STACK_SIZE]>,
}

unsafe impl Sync for KernelStack {}
unsafe impl Sync for UserStack {}

static KERNEL_STACK: KernelStack = KernelStack {
    data: UnsafeCell::new([0; KERNEL_STACK_SIZE]),
};

static USER_STACK: UserStack = UserStack {
    data: UnsafeCell::new([0; USER_STACK_SIZE]),
};

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.get() as usize + KERNEL_STACK_SIZE
    }

    fn push_context(&self, context: TrapContext) -> usize {
        let address =
            self.get_sp() - core::mem::size_of::<TrapContext>();

        unsafe {
            (address as *mut TrapContext).write(context);
        }

        address
    }
}

impl UserStack {
    fn get_bottom(&self) -> usize {
        self.data.get() as usize
    }

    fn get_sp(&self) -> usize {
        self.get_bottom() + USER_STACK_SIZE
    }

    unsafe fn clear(&self) {
        let start = self.get_bottom();
        let end = self.get_sp();

        for address in start..end {
            (address as *mut u8).write_volatile(0);
        }
    }
}

pub fn is_user_stack_range(start: usize, end: usize) -> bool {
    start >= USER_STACK.get_bottom()
        && end <= USER_STACK.get_sp()
}
