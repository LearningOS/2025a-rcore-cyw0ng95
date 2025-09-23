pub mod base_ext;
pub mod legacy_ext;

use core::arch::asm;

pub struct SbiRet {
    pub error: usize,
    pub value: usize,
}

fn sbi_call(eid :usize, 
        fid: usize,
        arg0: usize, 
        arg1: usize, 
        arg2: usize
    ) -> SbiRet {
    let (err, val);
    unsafe {
        asm!(
            "ecall",
            in("a0") arg0,
            in("a1") arg1,
            in("a2") arg2,
            in("a6") fid,
            in("a7") eid,
            lateout("a0") err,
            lateout("a1") val,
        );
    }
    SbiRet { error: err, value: val }
}
