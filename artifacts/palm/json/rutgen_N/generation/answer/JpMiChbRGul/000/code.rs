// Answer 0

#[test]
fn test_collect_str() {
    struct TestDisplay {
        content: String,
    }

    impl std::fmt::Display for TestDisplay {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.content)
        }
    }

    let value = TestDisplay { content: String::from("Hello, world!") };
    let result = collect_str(value);
    assert_eq!(result.unwrap(), "Hello, world!");
}

#[test]
fn test_collect_str_empty() {
    struct TestDisplay {
        content: String,
    }

    impl std::fmt::Display for TestDisplay {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.content)
        }
    }

    let value = TestDisplay { content: String::from("") };
    let result = collect_str(value);
    assert_eq!(result.unwrap(), "");
}

