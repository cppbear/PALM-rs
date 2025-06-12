unsafe fn promotable_even_to_vec(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> Vec<u8> {
    promotable_to_vec(data, ptr, len, |shared| {
        ptr_map(shared.cast(), |addr| addr & !KIND_MASK)
    })
}