unsafe fn shared_to_vec(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> Vec<u8> {
    shared_to_vec_impl(data.load(Ordering::Relaxed).cast(), ptr, len)
}