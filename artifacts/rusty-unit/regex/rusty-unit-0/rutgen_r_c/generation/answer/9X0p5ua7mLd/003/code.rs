// Answer 0

#[test]
fn test_set_range_start_greater_than_end() {
    let mut byte_class_set = ByteClassSet::new();
    let start: u8 = 2;
    let end: u8 = 1;

    let result = std::panic::catch_unwind(|| {
        byte_class_set.set_range(start, end);
    });

    assert!(result.is_err(), "set_range should panic when start > end");
}

#[test]
fn test_set_range_start_equals_end() {
    let mut byte_class_set = ByteClassSet::new();
    let start: u8 = 5;
    let end: u8 = 5;

    // This should not panic and should only set the specific index
    byte_class_set.set_range(start, end);
    
    // Check if the specific index is set to true
    assert!(byte_class_set.0[start as usize] == true, "Byte at index 5 should be set to true");
}

#[test]
fn test_set_range_start_zero() {
    let mut byte_class_set = ByteClassSet::new();
    let start: u8 = 0; // This should not trigger panic
    let end: u8 = 3;

    let result = std::panic::catch_unwind(|| {
        byte_class_set.set_range(start, end);
    });

    assert!(result.is_err(), "set_range should panic when start is 0 since it modifies start - 1");
}

#[test]
fn test_set_range_normal_case() {
    let mut byte_class_set = ByteClassSet::new();
    let start: u8 = 1;
    let end: u8 = 3;

    // This should not panic and should set the byte classes correctly
    byte_class_set.set_range(start, end);
    
    // After setting the range, the specified bytes should be 'true'
    assert!(byte_class_set.0[0] == true, "Byte at index 0 should be set to true");
    assert!(byte_class_set.0[3] == true, "Byte at index 3 should be set to true");
}

