fn invalid_ptr<T>(addr: usize) -> *mut T {
    let ptr = core::ptr::null_mut::<u8>().wrapping_add(addr);
    debug_assert_eq!(ptr as usize, addr);
    ptr.cast::<T>()
}