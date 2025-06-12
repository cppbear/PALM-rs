// Answer 0

#[derive(Debug)]
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
    assert_eq!(test_struct.case_insensitive(), true);
}

#[test]
fn test_case_insensitive_some_false() {
    let test_struct = TestStruct {
        case_insensitive: Some(false),
    };
    assert_eq!(test_struct.case_insensitive(), false);
}

#[test]
fn test_case_insensitive_none() {
    let test_struct = TestStruct {
        case_insensitive: None,
    };
    assert_eq!(test_struct.case_insensitive(), false);
}

