// Answer 0

#[test]
fn test_pattern_with_non_empty_string() {
    struct TestStruct {
        pattern: std::cell::RefCell<String>,
    }

    impl TestStruct {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: std::cell::RefCell::new(pattern.to_string()),
            }
        }

        fn pattern(&self) -> &str {
            self.pattern.borrow()
        }
    }

    let test_instance = TestStruct::new("a.*b");
    assert_eq!(test_instance.pattern(), "a.*b");
}

#[test]
fn test_pattern_with_empty_string() {
    struct TestStruct {
        pattern: std::cell::RefCell<String>,
    }

    impl TestStruct {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: std::cell::RefCell::new(pattern.to_string()),
            }
        }

        fn pattern(&self) -> &str {
            self.pattern.borrow()
        }
    }

    let test_instance = TestStruct::new("");
    assert_eq!(test_instance.pattern(), "");
}

#[test]
fn test_pattern_with_whitespace() {
    struct TestStruct {
        pattern: std::cell::RefCell<String>,
    }

    impl TestStruct {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: std::cell::RefCell::new(pattern.to_string()),
            }
        }

        fn pattern(&self) -> &str {
            self.pattern.borrow()
        }
    }

    let test_instance = TestStruct::new("   ");
    assert_eq!(test_instance.pattern(), "   ");
}

