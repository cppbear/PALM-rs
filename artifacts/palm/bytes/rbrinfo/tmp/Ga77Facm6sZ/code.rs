unsafe fn promotable_even_to_mut(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> BytesMut {
    promotable_to_mut(data, ptr, len, |shared| {
        ptr_map(shared.cast(), |addr| addr & !KIND_MASK)
    })
}