// Answer 0

#[test]
fn test_pattern() {
    struct Error {
        pattern: String,
    }

    impl Error {
        pub fn new(pattern: String) -> Self {
            Self { pattern }
        }

        pub fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let error_instance = Error::new("abc([xyz]?)".to_string());
    assert_eq!(error_instance.pattern(), "abc([xyz]?)");

    let error_instance_empty = Error::new("".to_string());
    assert_eq!(error_instance_empty.pattern(), "");
}

