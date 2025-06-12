// Answer 0

#[test]
fn test_set_range_valid_input() {
    let mut byte_class_set = ByteClassSet::new();
    byte_class_set.set_range(10, 20);
    
    assert_eq!(byte_class_set.0[9], true); // start-1 = 9
    assert_eq!(byte_class_set.0[20], true); // end = 20
}

#[test]
fn test_set_range_start_zero() {
    let mut byte_class_set = ByteClassSet::new();
    byte_class_set.set_range(0, 20);
    
    assert_eq!(byte_class_set.0[20], true); // end = 20
}

#[test]
#[should_panic]
fn test_set_range_start_greater_than_end() {
    let mut byte_class_set = ByteClassSet::new();
    byte_class_set.set_range(20, 10); // This should trigger the debug_assert
}

#[test]
fn test_set_range_max_range() {
    let mut byte_class_set = ByteClassSet::new();
    byte_class_set.set_range(0, 255);
    
    assert_eq!(byte_class_set.0[255], true); // end = 255
}

#[test]
fn test_set_range_no_effect_on_unset_bits() {
    let mut byte_class_set = ByteClassSet::new();
    byte_class_set.set_range(50, 60);
    
    assert_eq!(byte_class_set.0[0], false); // usage of a bit not set
    assert_eq!(byte_class_set.0[49], false); // usage of a bit not set
    assert_eq!(byte_class_set.0[61], false); // usage of a bit not set
    assert_eq!(byte_class_set.0[255], false); // usage of a bit not set
}

