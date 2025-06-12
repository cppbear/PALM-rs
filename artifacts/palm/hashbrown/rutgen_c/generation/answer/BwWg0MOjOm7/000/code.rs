// Answer 0

#[test]
fn test_offset_from_same_pointer() {
    unsafe {
        let ptr: *const i32 = &10;
        let result = offset_from(ptr, ptr);
        assert_eq!(result, 0);
    }
}

#[test]
fn test_offset_from_different_pointers() {
    unsafe {
        let a: i32 = 10;
        let b: i32 = 20;
        let ptr_a: *const i32 = &a;
        let ptr_b: *const i32 = &b;
        let result = offset_from(ptr_b, ptr_a);
        // Size of i32 is 4 bytes, so the offset should be non-zero and positive
        assert!(result > 0);
    }
}

#[test]
fn test_offset_from_null_pointer() {
    unsafe {
        let ptr: *const i32 = ptr::null();
        let result = offset_from(ptr, ptr);
        assert_eq!(result, 0);
    }
}

#[test]
fn test_offset_from_null_and_non_null_pointer() {
    unsafe {
        let a: i32 = 10;
        let ptr_a: *const i32 = &a;
        let ptr_null: *const i32 = ptr::null();
        let result = offset_from(ptr_null, ptr_a);
        assert!(result > 0);
    }
}

