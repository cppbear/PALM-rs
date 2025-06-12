pub unsafe fn mul_shift_all_64(
    m: u64,
    mul: &(u64, u64),
    j: u32,
    vp: *mut u64,
    vm: *mut u64,
    mm_shift: u32,
) -> u64 {
    ptr::write(vp, mul_shift_64(4 * m + 2, mul, j));
    ptr::write(vm, mul_shift_64(4 * m - 1 - mm_shift as u64, mul, j));
    mul_shift_64(4 * m, mul, j)
}