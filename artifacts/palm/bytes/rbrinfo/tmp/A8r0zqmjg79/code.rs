unsafe fn static_to_vec(_: &AtomicPtr<()>, ptr: *const u8, len: usize) -> Vec<u8> {
    let slice = slice::from_raw_parts(ptr, len);
    slice.to_vec()
}