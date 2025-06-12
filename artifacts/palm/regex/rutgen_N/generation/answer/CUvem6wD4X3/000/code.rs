// Answer 0

#[test]
fn test_into_byte_regex() {
    struct Executor;

    impl From<Executor> for re_bytes::Regex {
        fn from(_executor: Executor) -> Self {
            re_bytes::Regex::new("test").unwrap()
        }
    }

    let executor = Executor;
    let regex = executor.into_byte_regex();
    assert!(regex.is_match(b"test"));
}

#[test]
fn test_into_byte_regex_empty() {
    struct Executor;

    impl From<Executor> for re_bytes::Regex {
        fn from(_executor: Executor) -> Self {
            re_bytes::Regex::new("").unwrap()
        }
    }

    let executor = Executor;
    let regex = executor.into_byte_regex();
    assert!(regex.is_match(b""));
}

#[test]
fn test_into_byte_regex_invalid() {
    struct Executor;

    impl From<Executor> for re_bytes::Regex {
        fn from(_executor: Executor) -> Self {
            re_bytes::Regex::new("([a-z") // invalid regex
                .expect("Invalid regex provided");
        }
    }

    let executor = Executor;
    let result = std::panic::catch_unwind(|| {
        executor.into_byte_regex();
    });
    assert!(result.is_err());
}

