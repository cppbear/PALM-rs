// Answer 0

#[derive(Default)]
struct TestStruct {
    case_insensitive: Option<bool>,
}

impl TestStruct {
    fn case_insensitive(&self) -> bool {
        self.case_insensitive.unwrap_or(false)
    }
}

#[test]
fn test_case_insensitive_some_true() {
    let test_struct = TestStruct {
        case_insensitive: Some(true),
    };
    assert!(test_struct.case_insensitive());
}

#[test]
fn test_case_insensitive_some_false() {
    let test_struct = TestStruct {
        case_insensitive: Some(false),
    };
    assert!(!test_struct.case_insensitive());
}

#[test]
fn test_case_insensitive_none() {
    let test_struct = TestStruct {
        case_insensitive: None,
    };
    assert!(!test_struct.case_insensitive());
}

#[test]
fn test_case_insensitive_empty() {
    let test_struct = TestStruct::default();
    assert!(!test_struct.case_insensitive());
}

