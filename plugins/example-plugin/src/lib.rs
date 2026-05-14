#[no_mangle]
pub extern "C" fn process(input_ptr: *const u8, input_len: i32) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn alloc(size: i32) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}
