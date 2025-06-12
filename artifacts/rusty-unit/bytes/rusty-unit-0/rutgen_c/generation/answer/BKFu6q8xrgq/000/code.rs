// Answer 0

#[test]
fn test_invalid_ptr_valid_address() {
    let addr: usize = 0x1000;
    let ptr: *mut u32 = invalid_ptr::<u32>(addr);
    assert_eq!(ptr as usize, addr);
}

#[test]
fn test_invalid_ptr_zero_address() {
    let addr: usize = 0;
    let ptr: *mut u8 = invalid_ptr::<u8>(addr);
    assert_eq!(ptr as usize, addr);
}

#[should_panic]
fn test_invalid_ptr_out_of_bounds() {
    let addr: usize = usize::MAX; // Using MAX to induce panic on debug_assert
    let _ptr: *mut i64 = invalid_ptr::<i64>(addr);
}

