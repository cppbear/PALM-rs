// Answer 0

#[test]
fn test_invalid_ptr_zero() {
    let ptr: *mut u32 = invalid_ptr(0);
    assert_eq!(ptr as usize, 0);
}

#[test]
fn test_invalid_ptr_non_zero() {
    let addr: usize = 16;
    let ptr: *mut u32 = invalid_ptr(addr);
    assert_eq!(ptr as usize, addr);
}

#[test]
fn test_invalid_ptr_large_address() {
    let addr: usize = usize::MAX - 1;
    let ptr: *mut u64 = invalid_ptr(addr);
    assert_eq!(ptr as usize, addr);
}

