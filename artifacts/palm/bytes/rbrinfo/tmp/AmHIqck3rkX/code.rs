unsafe fn owned_to_mut(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> BytesMut {
    BytesMut::from_vec(owned_to_vec(data, ptr, len))
}