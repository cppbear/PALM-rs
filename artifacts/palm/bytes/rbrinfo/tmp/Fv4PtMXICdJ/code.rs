fn vptr(ptr: *mut u8) -> NonNull<u8> {
    if cfg!(debug_assertions) {
        NonNull::new(ptr).expect("Vec pointer should be non-null")
    } else {
        unsafe { NonNull::new_unchecked(ptr) }
    }
}