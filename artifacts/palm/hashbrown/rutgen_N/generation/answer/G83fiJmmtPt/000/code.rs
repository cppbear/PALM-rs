// Answer 0

#[test]
fn test_invalid_mut_valid_address() {
    let addr: usize = 0x1 as usize;
    let ptr: *mut u8 = invalid_mut::<u8>(addr);
    assert_eq!(ptr as usize, addr);
}

#[test]
#[should_panic]
fn test_invalid_mut_zero_address() {
    let addr: usize = 0;
    let _ptr: *mut u8 = invalid_mut::<u8>(addr);
}

#[test]
fn test_invalid_mut_large_address() {
    let addr: usize = usize::MAX;
    let ptr: *mut u64 = invalid_mut::<u64>(addr);
    assert_eq!(ptr as usize, addr);
}

