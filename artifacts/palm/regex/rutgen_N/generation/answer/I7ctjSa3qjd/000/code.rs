// Answer 0

#[test]
fn test_replace_append() {
    use regex::Regex;
    use regex::Captures;

    struct TestStruct {
        pattern: String,
    }

    impl TestStruct {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
            }
        }
    }

    let mut re = TestStruct::new(r"(\w+)");
    let caps = Regex::new(r"(\w+)").unwrap().captures("Hello").unwrap();
    let mut dst = String::new();

    re.replace_append(&caps, &mut dst);
    
    assert_eq!(dst, "Hello");
}

#[test]
#[should_panic]
fn test_replace_append_invalid_captures() {
    use regex::Regex;

    struct TestStruct {
        pattern: String,
    }

    impl TestStruct {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
            }
        }
    }

    let mut re = TestStruct::new(r"(\w+)");
    let caps = Regex::new(r"(\w+)").unwrap().captures("123").unwrap();
    let mut dst = String::new();

    // Intentionally using incorrect captures to trigger panic
    caps.expand("xy", &mut dst);
}

