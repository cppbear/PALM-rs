// Answer 0

#[test]
fn test_ptr_map_simple() {
    let ptr: *mut u8 = 0x1000 as *mut u8; // Example pointer
    let result = ptr_map(ptr, |x| x + 1);
    assert_eq!(result, (0x1000 + 1) as *mut u8);
}

#[test]
fn test_ptr_map_zero() {
    let ptr: *mut u8 = 0x0 as *mut u8; // Null pointer
    let result = ptr_map(ptr, |x| x + 5);
    assert_eq!(result, (0x0 + 5) as *mut u8);
}

#[test]
fn test_ptr_map_large_value() {
    let ptr: *mut u8 = 0xFFFFFFFF as *mut u8; // Large pointer value
    let result = ptr_map(ptr, |x| x.wrapping_add(1));
    assert_eq!(result, (0xFFFFFFFF + 1) as *mut u8);
}

#[test]
fn test_ptr_map_negative_offset() {
    let ptr: *mut u8 = 0x1000 as *mut u8; // Example pointer
    let result = ptr_map(ptr, |x| x.wrapping_sub(100));
    assert_eq!(result, (0x1000.wrapping_sub(100)) as *mut u8);
}

