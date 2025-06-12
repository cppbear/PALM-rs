// Answer 0

#[derive(Debug)]
struct TestStruct {
    pattern: String,
}

impl TestStruct {
    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}

#[test]
fn test_pattern_non_empty() {
    let test_instance = TestStruct {
        pattern: "abc".to_string(),
    };
    assert_eq!(test_instance.pattern(), "abc");
}

#[test]
fn test_pattern_empty() {
    let test_instance = TestStruct {
        pattern: "".to_string(),
    };
    assert_eq!(test_instance.pattern(), "");
}

#[test]
fn test_pattern_whitespace() {
    let test_instance = TestStruct {
        pattern: "   ".to_string(),
    };
    assert_eq!(test_instance.pattern(), "   ");
}

#[test]
fn test_pattern_special_characters() {
    let test_instance = TestStruct {
        pattern: "!@#$%^&*()_+".to_string(),
    };
    assert_eq!(test_instance.pattern(), "!@#$%^&*()_+");
}

