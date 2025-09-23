use crate::sbi::SbiRet;
use crate::sbi::sbi_call;

const SBI_EID_SET_TIMER: usize = 0;
const SBI_EID_CONSOLE_PUTCHAR: usize = 1;
const SBI_EID_CONSOLE_GETCHAR: usize = 2;
const SBI_EID_CLEAR_IPI: usize = 3;
const SBI_EID_SEND_IPI: usize = 4;
const SBI_EID_REMOTE_FENCE_I: usize = 5;
const SBI_EID_REMOTE_SFENCE_VMA: usize = 6;
const SBI_EID_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_EID_SHUTDOWN: usize = 8;

pub fn sbi_set_timer(stime_value: usize) {
    let SbiRet { error, value: _ } = sbi_call(SBI_EID_SET_TIMER, 0, stime_value, 0, 0);
    if error != 0 {
        panic!("SBI call failed with error code {}", error);
    }
}

pub fn sbi_console_putchar(c: usize) {
    let SbiRet { error, value: _ } = sbi_call(SBI_EID_CONSOLE_PUTCHAR, 0, c, 0, 0);
    // if error != 0 {
    //     panic!("SBI call failed with error code {}", error);
    // }
}

pub fn sbi_console_getchar() -> isize {
    let SbiRet { error, value } = sbi_call(SBI_EID_CONSOLE_GETCHAR, 0, 0, 0, 0);
    if error != 0 {
        -1
    } else {
        value as isize
    }
}

pub fn sbi_clear_ipi() {
    let SbiRet { error, value: _ } = sbi_call(SBI_EID_CLEAR_IPI, 0, 0, 0, 0);
    if error != 0 {
        panic!("SBI call failed with error code {}", error);
    }
}

pub fn sbi_send_ipi(hart_mask: usize) {
    let SbiRet { error, value: _ } = sbi_call(SBI_EID_SEND_IPI, 0, hart_mask, 0, 0);
    if error != 0 {
        panic!("SBI call failed with error code {}", error);
    }
}

pub fn sbi_remote_fence_i(hart_mask: usize) {
    let SbiRet { error, value: _ } = sbi_call(SBI_EID_REMOTE_FENCE_I, 0, hart_mask, 0, 0);
    if error != 0 {
        panic!("SBI call failed with error code {}", error);
    }
}

pub fn sbi_remote_sfence_vma(hart_mask: usize, start_addr: usize, size: usize) {
    let SbiRet { error, value: _ } = sbi_call(SBI_EID_REMOTE_SFENCE_VMA, 0, hart_mask, start_addr, size);
    if error != 0 {
        panic!("SBI call failed with error code {}", error);
    }
}

pub fn sbi_remote_sfence_vma_asid(hart_mask: usize, start_addr: usize, size: usize, asid: usize) {
    let SbiRet { error, value: _ } = sbi_call(SBI_EID_REMOTE_SFENCE_VMA_ASID, 0, hart_mask, start_addr, size | (asid << 56));
    if error != 0 {
        panic!("SBI call failed with error code {}", error);
    }
}

pub fn sbi_shutdown() -> ! {
    sbi_call(SBI_EID_SHUTDOWN, 0, 0, 0, 0);
    panic!("It should shutdown!");
}