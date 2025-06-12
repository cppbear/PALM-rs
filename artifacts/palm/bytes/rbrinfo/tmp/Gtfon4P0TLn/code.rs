fn ptr_map<F>(ptr: *mut u8, f: F) -> *mut u8
where
    F: FnOnce(usize) -> usize,
{
    let old_addr = ptr as usize;
    let new_addr = f(old_addr);
    new_addr as *mut u8
}