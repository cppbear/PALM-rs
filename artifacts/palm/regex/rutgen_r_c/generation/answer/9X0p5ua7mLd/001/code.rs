// Answer 0

#[test]
fn test_set_range_equal_start_end() {
    let mut byte_class_set = ByteClassSet::new();
    byte_class_set.set_range(1, 1);
    assert_eq!(byte_class_set.0[0], true);
    assert_eq!(byte_class_set.0[1], true);
}

#[test]
fn test_set_range_start_greater_than_zero() {
    let mut byte_class_set = ByteClassSet::new();
    byte_class_set.set_range(2, 5);
    assert_eq!(byte_class_set.0[1], true);
    assert_eq!(byte_class_set.0[2], true);
    assert_eq!(byte_class_set.0[3], true);
    assert_eq!(byte_class_set.0[4], true);
    assert_eq!(byte_class_set.0[5], true);
}

#[test]
#[should_panic]
fn test_set_range_panic_start_greater_than_end() {
    let mut byte_class_set = ByteClassSet::new();
    byte_class_set.set_range(2, 1);
}

#[test]
#[should_panic]
fn test_set_range_panic_start_equal_zero() {
    let mut byte_class_set = ByteClassSet::new();
    byte_class_set.set_range(0, 0);
}

