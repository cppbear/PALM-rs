// Answer 0

#[test]
fn test_collect_str_with_valid_display() {
    struct TestStruct<'a> {
        value: &'a str,
    }

    impl std::fmt::Display for TestStruct<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    let test_instance = TestStruct { value: "Hello, world!" };
    let result = collect_str(&test_instance);
    assert!(result.is_ok());
}

#[test]
fn test_collect_str_with_empty_string() {
    struct TestStruct<'a> {
        value: &'a str,
    }

    impl std::fmt::Display for TestStruct<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    let test_instance = TestStruct { value: "" };
    let result = collect_str(&test_instance);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_collect_str_with_non_display() {
    struct NonDisplay;

    let non_display_instance = NonDisplay;
    let result = collect_str(&non_display_instance);
    assert!(result.is_err()); // Expecting panic due to non-Display.
}

