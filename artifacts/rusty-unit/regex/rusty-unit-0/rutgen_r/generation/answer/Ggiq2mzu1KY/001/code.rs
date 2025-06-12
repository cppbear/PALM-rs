// Answer 0

#[test]
fn test_is_match_true() {
    struct TestStruct(u8);

    let test_instance = TestStruct(1);
    assert!(test_instance.is_match());
}

#[test]
fn test_is_match_false() {
    struct TestStruct(u8);

    let test_instance = TestStruct(0);
    assert!(!test_instance.is_match());
}

#[test]
fn test_is_match_boundary_high() {
    struct TestStruct(u8);

    let test_instance = TestStruct(255);
    assert!(test_instance.is_match());
}

#[test]
fn test_is_match_boundary_low() {
    struct TestStruct(u8);

    let test_instance = TestStruct(2);
    assert!(!test_instance.is_match());
}

#[test]
fn test_is_match_with_negative() {
    struct TestStruct(i8);

    let test_instance = TestStruct(-1);
    assert_eq!(test_instance.is_match(), false);
}

