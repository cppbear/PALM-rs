// Answer 0

#[test]
fn test_is_match_true() {
    struct TestStruct(u32);

    let instance = TestStruct(0b0000000_1);
    assert!(instance.is_match());
}

#[test]
fn test_is_match_false() {
    struct TestStruct(u32);

    let instance = TestStruct(0b0000000_0);
    assert!(!instance.is_match());
}

#[test]
fn test_is_match_boundary() {
    struct TestStruct(u32);

    let instance = TestStruct(0b0000000_1);
    assert!(instance.is_match());

    let instance_zero = TestStruct(0b0000000_0);
    assert!(!instance_zero.is_match());
}

