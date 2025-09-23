use crate::sbi::SbiRet;
use crate::sbi::sbi_call;

const SBI_EID_BASE_EXT: usize = 1;
const SBI_FID_GET_SPEC_VERSION: usize = 0;
const SBI_FID_GET_IMPLEMENTATION_ID: usize = 1;
const SBI_FID_GET_IMPLEMENTATION_VERSION: usize = 2;
const SBI_FID_GET_MARCHID: usize = 5;

pub fn sbi_get_impl_id() -> usize {
    let SbiRet { error, value } = sbi_call(SBI_EID_BASE_EXT, SBI_FID_GET_IMPLEMENTATION_ID, 0, 0, 0);
    if error != 0 {
        panic!("SBI call failed with error code {}", error);
    }

    value
}