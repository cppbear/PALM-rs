fn without_provenance(ptr: usize) -> *const u8 {
    core::ptr::null::<u8>().wrapping_add(ptr)
}