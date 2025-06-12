// Answer 0

#[test]
fn test_into_byte_regex_valid_input() {
    struct ByteExecutor {
        pattern: String,
    }

    impl ByteExecutor {
        pub fn new(pattern: impl Into<String>) -> Self {
            Self {
                pattern: pattern.into(),
            }
        }

        pub fn into_byte_regex(self) -> re_bytes::Regex {
            re_bytes::Regex::from(self.pattern)
        }
    }

    let executor = ByteExecutor::new("abc");
    let regex = executor.into_byte_regex();
    assert!(regex.is_match(b"abc"));
}

#[test]
fn test_into_byte_regex_empty_input() {
    struct ByteExecutor {
        pattern: String,
    }

    impl ByteExecutor {
        pub fn new(pattern: impl Into<String>) -> Self {
            Self {
                pattern: pattern.into(),
            }
        }

        pub fn into_byte_regex(self) -> re_bytes::Regex {
            re_bytes::Regex::from(self.pattern)
        }
    }

    let executor = ByteExecutor::new("");
    let regex = executor.into_byte_regex();
    assert!(regex.is_match(b""));
}

#[test]
#[should_panic]
fn test_into_byte_regex_invalid_pattern() {
    struct ByteExecutor {
        pattern: String,
    }

    impl ByteExecutor {
        pub fn new(pattern: impl Into<String>) -> Self {
            Self {
                pattern: pattern.into(),
            }
        }

        pub fn into_byte_regex(self) -> re_bytes::Regex {
            re_bytes::Regex::from(self.pattern)
        }
    }

    let executor = ByteExecutor::new("["); // Invalid regex pattern
    executor.into_byte_regex();
}

