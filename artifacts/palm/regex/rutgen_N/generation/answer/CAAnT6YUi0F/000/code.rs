// Answer 0

#[test]
fn test_set_match() {
    struct TestStruct(u8);
    
    let mut test_instance = TestStruct(0b0000000_0);
    test_instance.set_match();
    assert_eq!(test_instance.0, 0b0000000_1);
}

#[test]
fn test_set_match_increments() {
    struct TestStruct(u8);
    
    let mut test_instance = TestStruct(0b0000000_0);
    test_instance.set_match();
    test_instance.set_match();
    assert_eq!(test_instance.0, 0b0000000_1);
}

