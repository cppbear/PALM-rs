// Answer 0

#[test]
fn test_ptr_map_valid_pointer() {
    let value: u8 = 42;
    let ptr: *mut u8 = &value as *const u8 as *mut u8;

    unsafe {
        let result = ptr_map(ptr, |old_addr| old_addr + 4);
        assert_eq!(result as usize, ptr as usize + 4);
    }
}

#[test]
fn test_ptr_map_zero_offset() {
    let value: u8 = 42;
    let ptr: *mut u8 = &value as *const u8 as *mut u8;

    unsafe {
        let result = ptr_map(ptr, |old_addr| old_addr);
        assert_eq!(result as usize, ptr as usize);
    }
}

#[test]
fn test_ptr_map_negative_offset() {
    let value: u8 = 42;
    let ptr: *mut u8 = &value as *const u8 as *mut u8;

    unsafe {
        let result = ptr_map(ptr, |old_addr| old_addr.wrapping_sub(4));
        assert_eq!(result as usize, ptr as usize.wrapping_sub(4));
    }
}

#[should_panic]
#[test]
fn test_ptr_map_potentially_invalid_pointer() {
    let invalid_ptr: *mut u8 = 0 as *mut u8;

    unsafe {
        ptr_map(invalid_ptr, |old_addr| old_addr + 4);
    }
}

