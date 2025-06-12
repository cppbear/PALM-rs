// Answer 0

#[test]
fn test_only_utf8_true() {
    struct TestStruct {
        only_utf8: bool,
    }

    let test_instance = TestStruct { only_utf8: true };
    assert_eq!(test_instance.only_utf8(), true);
}

#[test]
fn test_only_utf8_false() {
    struct TestStruct {
        only_utf8: bool,
    }

    let test_instance = TestStruct { only_utf8: false };
    assert_eq!(test_instance.only_utf8(), false);
}

