unsafe fn owned_to_vec(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> Vec<u8> {
    let slice = slice::from_raw_parts(ptr, len);
    let vec = slice.to_vec();
    owned_drop_impl(data.load(Ordering::Relaxed));
    vec
}