use sha2::{Sha256, Digest};
use std::slice;

#[link_section = ".custom_memory"]
static mut MEMORY: [u8; 110 * 1024 * 1024] = [0; 110 * 1024 * 1024];

#[no_mangle]
pub extern "C" fn double_sha256(input_ptr: *const u8, input_len: usize, output_ptr: *mut u8) {
    let input = unsafe { slice::from_raw_parts(input_ptr, input_len) };
    let first_hash = Sha256::digest(input);
    let second_hash = Sha256::digest(&first_hash);

    unsafe {
        std::ptr::copy_nonoverlapping(second_hash.as_ptr(), output_ptr, 32);
    }
}
