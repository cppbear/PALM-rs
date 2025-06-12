// Answer 0

#[test]
fn test_byte_return_value() {
    struct TestStruct {
        byte: Option<u8>,
    }

    let instance = TestStruct { byte: Some(100) };
    assert_eq!(instance.byte(), Some(100));
}

#[test]
fn test_byte_return_none() {
    struct TestStruct {
        byte: Option<u8>,
    }

    let instance = TestStruct { byte: None };
    assert_eq!(instance.byte(), None);
}

