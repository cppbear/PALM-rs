// Answer 0

#[test]
fn test_invalid_uri_too_long() {
    struct TestInvalidUri;

    impl TestInvalidUri {
        fn s(&self) -> &str {
            InvalidUri(ErrorKind::TooLong).s()
        }
    }

    let invalid_uri_instance = TestInvalidUri;
    assert_eq!(invalid_uri_instance.s(), "uri too long");
}

