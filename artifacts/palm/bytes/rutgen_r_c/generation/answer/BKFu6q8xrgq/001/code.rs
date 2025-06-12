// Answer 0

#[test]
fn test_invalid_ptr_dangling_pointer_zero_address() {
    let addr: usize = 0;
    let result: *mut u8 = invalid_ptr::<u8>(addr);
    assert_eq!(result as usize, addr);
}

#[test]
fn test_invalid_ptr_dangling_pointer_non_zero_address() {
    let addr: usize = 1234;
    let result: *mut u8 = invalid_ptr::<u8>(addr);
    assert_eq!(result as usize, addr);
}

#[test]
fn test_invalid_ptr_dangling_pointer_large_address() {
    let addr: usize = usize::MAX;
    let result: *mut u8 = invalid_ptr::<u8>(addr);
    assert_eq!(result as usize, addr);
}

#[test]
#[should_panic]
fn test_invalid_ptr_fail_on_negative_address() {
    let addr: usize = isize::MAX as usize + 1; // simulate a negative address
    invalid_ptr::<u8>(addr); // This should panic due to invalid pointer
}

