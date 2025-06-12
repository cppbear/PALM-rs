// Answer 0

#[test]
fn test_offset_from() {
    let value1: i32 = 10;
    let value2: i32 = 20;

    let ptr1: *const i32 = &value1;
    let ptr2: *const i32 = &value2;

    unsafe {
        // Testing pointer with different values
        assert_eq!(offset_from(ptr1, ptr2), 4); // ptr1 is 4 bytes ahead of ptr2
        assert_eq!(offset_from(ptr2, ptr1), usize::MAX - 4); // the other direction, considering overflow
    }
}

#[test]
fn test_offset_from_same_pointer() {
    let value: i32 = 10;

    let ptr: *const i32 = &value;

    unsafe {
        // Testing pointer to itself
        assert_eq!(offset_from(ptr, ptr), 0); // offset from itself should be zero
    }
}

#[test]
#[should_panic]
fn test_offset_from_null_pointer() {
    let ptr1: *const i32 = core::ptr::null();
    let value: i32 = 20;
    let ptr2: *const i32 = &value;

    unsafe {
        // This should panic as it's calling offset_from with a null pointer
        let _ = offset_from(ptr1, ptr2);
    }
}

#[test]
fn test_offset_from_large_offsets() {
    let array: [i32; 10] = [0; 10];
    
    let ptr1: *const i32 = &array[0];
    let ptr2: *const i32 = &array[9];

    unsafe {
        // Testing large offsets within the same array
        assert_eq!(offset_from(ptr1, ptr2), 36); // 9 * 4 bytes (size of i32)
        assert_eq!(offset_from(ptr2, ptr1), 36); // negative offset would still give same bytes, leading to unsigned interpretation
    }
}

