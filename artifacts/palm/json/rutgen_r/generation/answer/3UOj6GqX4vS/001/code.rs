// Answer 0

#[test]
fn test_end_array_value() {
    use std::io::{Cursor, Write};

    struct TestStruct {
        has_value: bool,
    }

    let mut test_struct = TestStruct { has_value: false };
    let mut cursor = Cursor::new(Vec::new());

    let result = test_struct.end_array_value(&mut cursor);

    assert!(result.is_ok());
    assert!(test_struct.has_value);
}

