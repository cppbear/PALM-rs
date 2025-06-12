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
        pattern: String::from("a*b+c?"),
    };
    assert_eq!(test_instance.pattern(), "a*b+c?");
}

#[test]
fn test_pattern_empty() {
    let test_instance = TestStruct {
        pattern: String::from(""),
    };
    assert_eq!(test_instance.pattern(), "");
}

#[test]
fn test_pattern_whitespace() {
    let test_instance = TestStruct {
        pattern: String::from("   "),
    };
    assert_eq!(test_instance.pattern(), "   ");
}

#[test]
fn test_pattern_special_characters() {
    let test_instance = TestStruct {
        pattern: String::from("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"),
    };
    assert_eq!(test_instance.pattern(), "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$");
}

