//! Batch subsystem

use core::arch::asm;

use lazy_static::lazy_static;
use log::{debug, trace};

use crate::{sbi::shutdown, sync::UPSafeCell, trap::TrapContext};

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;
const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};

static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

impl KernelStack {
    /// Because Stack in RISC-V is downward grossing, we calculate with data ptr + Stack size
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
            cx_ptr.as_mut().unwrap()
        }
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

// `lazy_static!` help us initialize a global variable at the first time it is used.
lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" {
            fn _num_app();
        }
            // Get app_num array start ptr
            let num_app_ptr = _num_app as usize as *const usize;
            // Read app_num array start address
            let num_app = num_app_ptr.read_volatile();
            // Init an array of each app's start address and the last app's end address.
            let mut app_start = [0; MAX_APP_NUM + 1];
            // Add 1 to num_app_ptr to start from the first app's address, read raw apps' start.
            let app_start_raw = core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            // Apply these addresses to app_start array.
            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManager{
                num_app,
                current_app: 0,
                app_start
            }
        })
    };
}

/// Batch OS app manager.
struct AppManager {
    /// Num of apps.
    num_app: usize,
    /// Current running app.
    current_app: usize,
    /// Each app's start address and last app's end address array.
    app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    /// Log app information
    pub fn log_app_info(&self) {
        debug!("num_app = {}", self.num_app);
        for i in 0..self.num_app {
            debug!(
                "app_{} [{:#x}, {:#x}]",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            )
        }
    }

    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            trace!("All applications completed!");
            shutdown(false);
        }
        trace!("Loading app_{}", app_id);
        // Clear app area.
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );
        // Load new app to app dest.
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len())
            .copy_from_slice(app_src);
        // Memory fence about fetching the instruction memory
        // It is guaranteed that a subsequent instruction fetch must observes all previous writes
        // to the instruction cache memory.
        // Therefore, fence.i must be executed after we have loaded the code of the next app into
        // the instruction memory cache.
        asm!("fence.i")
    }

    /// Get current app
    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    /// Move to next app
    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }
}

/// Init batch subsystem
pub fn init() {
    log_app_info()
}

/// Log app info
pub fn log_app_info() {
    APP_MANAGER.exclusive_access().log_app_info();
}

/// Run next app
pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    unsafe {
        app_manager.load_app(current_app);
    }
    app_manager.move_to_next_app();
    drop(app_manager);
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp(),
        )) as *const _ as usize);
    }
    unreachable!("restore will call ret, thus this will never reach");
}
