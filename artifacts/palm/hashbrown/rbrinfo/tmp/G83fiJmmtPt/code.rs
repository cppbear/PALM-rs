fn invalid_mut<T>(addr: usize) -> *mut T {
    unsafe { core::mem::transmute(addr) }
}