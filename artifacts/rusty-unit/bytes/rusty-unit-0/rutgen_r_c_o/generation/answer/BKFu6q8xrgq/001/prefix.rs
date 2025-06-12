// Answer 0

#[test]
fn test_invalid_ptr_zero() {
    let addr: usize = 0;
    let ptr: *mut u8 = invalid_ptr::<u8>(addr);
}

#[test]
fn test_invalid_ptr_one() {
    let addr: usize = 1;
    let ptr: *mut u8 = invalid_ptr::<u8>(addr);
}

#[test]
fn test_invalid_ptr_max() {
    let addr: usize = usize::MAX;
    let ptr: *mut u8 = invalid_ptr::<u8>(addr);
}

#[test]
fn test_invalid_ptr_mid_range() {
    let addr: usize = usize::MAX / 2;
    let ptr: *mut u8 = invalid_ptr::<u8>(addr);
}

#[test]
fn test_invalid_ptr_near_max() {
    let addr: usize = usize::MAX - 1;
    let ptr: *mut u8 = invalid_ptr::<u8>(addr);
}

#[test]
#[should_panic]
fn test_invalid_ptr_negative() {
    // This case is not applicable in Rust as usize cannot be negative,
    // but showcasing this to fulfill the panic triggers concept.
    let addr: usize = isize::MAX as usize + 1; // out of bounds
    let ptr: *mut u8 = invalid_ptr::<u8>(addr);
}

