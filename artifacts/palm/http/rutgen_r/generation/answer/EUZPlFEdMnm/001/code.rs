// Answer 0

#[test]
fn test_fmt_valid_case() {
    struct Wrapper(String);
    
    impl std::fmt::Display for Wrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    
    let my_wrapper = Wrapper("test string".to_string());
    let result = my_wrapper.to_string();
    
    assert_eq!(result, "test string");
}

#[test]
fn test_fmt_empty_string() {
    struct Wrapper(String);
    
    impl std::fmt::Display for Wrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    
    let my_wrapper = Wrapper("".to_string());
    let result = my_wrapper.to_string();
    
    assert_eq!(result, "");
}

#[test]
#[should_panic]
fn test_fmt_panic_condition() {
    struct Wrapper(String);
    
    impl std::fmt::Display for Wrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.0.is_empty() {
                panic!("Panic triggered for empty string");
            }
            write!(f, "{}", self.0)
        }
    }
    
    let my_wrapper = Wrapper("".to_string());
    let _ = my_wrapper.to_string(); // Should trigger a panic
}

