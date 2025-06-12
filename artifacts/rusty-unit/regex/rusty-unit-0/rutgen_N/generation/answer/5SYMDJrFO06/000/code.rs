// Answer 0

#[derive(Debug)]
struct TestStruct {
    multi_line: Option<bool>,
}

impl TestStruct {
    fn multi_line(&self) -> bool {
        self.multi_line.unwrap_or(false)
    }
}

#[test]
fn test_multi_line_some_true() {
    let test_instance = TestStruct {
        multi_line: Some(true),
    };
    assert_eq!(test_instance.multi_line(), true);
}

#[test]
fn test_multi_line_some_false() {
    let test_instance = TestStruct {
        multi_line: Some(false),
    };
    assert_eq!(test_instance.multi_line(), false);
}

#[test]
fn test_multi_line_none() {
    let test_instance = TestStruct {
        multi_line: None,
    };
    assert_eq!(test_instance.multi_line(), false);
}

