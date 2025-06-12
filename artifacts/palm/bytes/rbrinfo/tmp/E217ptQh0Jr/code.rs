unsafe fn free_boxed_slice(buf: *mut u8, offset: *const u8, len: usize) {
    let cap = offset_from(offset, buf) + len;
    dealloc(buf, Layout::from_size_align(cap, 1).unwrap())
}