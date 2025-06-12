// Answer 0

#[test]
fn test_as_str_ok() {
    struct TestStatusCode(u16);
    impl TestStatusCode {
        fn as_str(&self) -> &str {
            let offset = (self.0 - 100) as usize;
            let offset = offset * 3;
            unsafe { CODE_DIGITS.get_unchecked(offset..offset + 3) }
        }
    }
    
    let status = TestStatusCode(200);
    assert_eq!(status.as_str(), "200");
}

#[test]
fn test_as_str_informational() {
    struct TestStatusCode(u16);
    impl TestStatusCode {
        fn as_str(&self) -> &str {
            let offset = (self.0 - 100) as usize;
            let offset = offset * 3;
            unsafe { CODE_DIGITS.get_unchecked(offset..offset + 3) }
        }
    }
    
    let status = TestStatusCode(100);
    assert_eq!(status.as_str(), "100");
}

#[test]
fn test_as_str_client_error() {
    struct TestStatusCode(u16);
    impl TestStatusCode {
        fn as_str(&self) -> &str {
            let offset = (self.0 - 100) as usize;
            let offset = offset * 3;
            unsafe { CODE_DIGITS.get_unchecked(offset..offset + 3) }
        }
    }
    
    let status = TestStatusCode(404);
    assert_eq!(status.as_str(), "404");
}

#[test]
fn test_as_str_server_error() {
    struct TestStatusCode(u16);
    impl TestStatusCode {
        fn as_str(&self) -> &str {
            let offset = (self.0 - 100) as usize;
            let offset = offset * 3;
            unsafe { CODE_DIGITS.get_unchecked(offset..offset + 3) }
        }
    }
    
    let status = TestStatusCode(500);
    assert_eq!(status.as_str(), "500");
}

