#![no_std]
#![no_main]

mod lang_item;
mod sbi;

core::arch::global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    clear_bss();
    sbi::shutdown();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

const SYSCALL_EXIT: usize = 93;

// rust syscall interface
// id: syscall number
// args: up to 3 arguments
// return: syscall return value
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
        );
    }
    ret
}

pub fn sys_exit(xstate : i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

const SYSCALL_WRITE: usize = 64;

pub fn sys_write(fd: usize, buf: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buf.as_ptr() as usize, buf.len()])
}