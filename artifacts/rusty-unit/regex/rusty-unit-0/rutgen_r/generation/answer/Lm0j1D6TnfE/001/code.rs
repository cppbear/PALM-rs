// Answer 0

#[derive(Default)]
struct TestStruct {
    dot_matches_new_line: Option<bool>,
}

impl TestStruct {
    fn dot_matches_new_line(&self) -> bool {
        self.dot_matches_new_line.unwrap_or(false)
    }
}

#[test]
fn test_dot_matches_new_line_some_true() {
    let test_struct = TestStruct {
        dot_matches_new_line: Some(true),
    };
    assert_eq!(test_struct.dot_matches_new_line(), true);
}

#[test]
fn test_dot_matches_new_line_some_false() {
    let test_struct = TestStruct {
        dot_matches_new_line: Some(false),
    };
    assert_eq!(test_struct.dot_matches_new_line(), false);
}

#[test]
fn test_dot_matches_new_line_none() {
    let test_struct = TestStruct {
        dot_matches_new_line: None,
    };
    assert_eq!(test_struct.dot_matches_new_line(), false);
}

