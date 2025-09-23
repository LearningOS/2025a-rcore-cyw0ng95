#![no_std]
#![no_main]

use log::info;

mod lang_item;
mod sbi;

#[macro_use]
mod console;
mod logging;

core::arch::global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    clear_bss();
    logging::init();
    info!("Hello, world!");
    sbi::legacy_ext::sbi_shutdown();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

fn print_num(mut num: usize) {
    if num == 0 {
        sbi::legacy_ext::sbi_console_putchar(b'0' as usize);
        return;
    }
    let mut buf = [0u8; 20];
    let mut i = buf.len();
    while num > 0 {
        i -= 1;
        buf[i] = b'0' + (num % 10) as u8;
        num /= 10;
    }
    for &ch in &buf[i..] {
        sbi::legacy_ext::sbi_console_putchar(ch as usize);
    }
}