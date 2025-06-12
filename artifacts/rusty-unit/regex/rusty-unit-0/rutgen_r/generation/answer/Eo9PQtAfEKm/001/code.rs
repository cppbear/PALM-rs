// Answer 0

#[test]
fn test_is_word_true() {
    struct TestStruct(u8);
    let test_instance = TestStruct(0b000000_1_0);
    assert!(test_instance.is_word());
}

#[test]
fn test_is_word_false() {
    struct TestStruct(u8);
    let test_instance = TestStruct(0b000000_0_0);
    assert!(!test_instance.is_word());
}

#[test]
fn test_is_word_another_false() {
    struct TestStruct(u8);
    let test_instance = TestStruct(0b000000_0_1);
    assert!(!test_instance.is_word());
}

#[test]
fn test_is_word_boundary() {
    struct TestStruct(u8);
    let test_instance = TestStruct(0b000000_1_1);
    assert!(test_instance.is_word());
}

#[test]
fn test_is_word_no_bits() {
    struct TestStruct(u8);
    let test_instance = TestStruct(0b000000_0_0);
    assert!(!test_instance.is_word());
}

