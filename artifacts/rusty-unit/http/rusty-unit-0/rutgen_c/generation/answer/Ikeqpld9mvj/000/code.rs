// Answer 0

#[test]
fn test_is_server_error_with_valid_server_error() {
    struct TestStatusCode(NonZeroU16);
    
    impl TestStatusCode {
        fn is_server_error(&self) -> bool {
            (500..600).contains(&self.0.get())
        }
    }

    let status_code = TestStatusCode(NonZeroU16::new(500).unwrap());
    assert!(status_code.is_server_error());

    let status_code = TestStatusCode(NonZeroU16::new(599).unwrap());
    assert!(status_code.is_server_error());
}

#[test]
fn test_is_server_error_with_non_server_error() {
    struct TestStatusCode(NonZeroU16);
    
    impl TestStatusCode {
        fn is_server_error(&self) -> bool {
            (500..600).contains(&self.0.get())
        }
    }

    let status_code = TestStatusCode(NonZeroU16::new(400).unwrap());
    assert!(!status_code.is_server_error());

    let status_code = TestStatusCode(NonZeroU16::new(600).unwrap());
    assert!(!status_code.is_server_error());
}

