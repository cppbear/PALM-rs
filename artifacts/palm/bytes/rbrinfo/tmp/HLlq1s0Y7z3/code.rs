unsafe fn promotable_odd_to_mut(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> BytesMut {
    promotable_to_mut(data, ptr, len, |shared| shared.cast())
}