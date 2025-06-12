// Answer 0

#[derive(Default)]
struct TestStruct {
    multi_line: Option<bool>,
}

#[test]
fn test_multi_line_some_true() {
    let test_struct = TestStruct {
        multi_line: Some(true),
    };
    assert_eq!(test_struct.multi_line(), true);
}

#[test]
fn test_multi_line_some_false() {
    let test_struct = TestStruct {
        multi_line: Some(false),
    };
    assert_eq!(test_struct.multi_line(), false);
}

#[test]
fn test_multi_line_none() {
    let test_struct = TestStruct {
        multi_line: None,
    };
    assert_eq!(test_struct.multi_line(), false);
}

