unsafe fn rebuild_vec(ptr: *mut u8, mut len: usize, mut cap: usize, off: usize) -> Vec<u8> {
    let ptr = ptr.sub(off);
    len += off;
    cap += off;

    Vec::from_raw_parts(ptr, len, cap)
}