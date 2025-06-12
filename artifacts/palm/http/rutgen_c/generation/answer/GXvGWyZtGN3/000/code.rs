// Answer 0

#[test]
fn test_is_success_with_success_code() {
    struct TestCode(u16);
    impl TestCode {
        fn is_success(&self) -> bool {
            (200..300).contains(&self.0)
        }
    }

    let status_code = TestCode(200);
    assert!(status_code.is_success());
    
    let status_code = TestCode(250);
    assert!(status_code.is_success());
    
    let status_code = TestCode(299);
    assert!(status_code.is_success());
}

#[test]
fn test_is_success_with_non_success_code() {
    struct TestCode(u16);
    impl TestCode {
        fn is_success(&self) -> bool {
            (200..300).contains(&self.0)
        }
    }

    let status_code = TestCode(199);
    assert!(!status_code.is_success());
    
    let status_code = TestCode(300);
    assert!(!status_code.is_success());
    
    let status_code = TestCode(404);
    assert!(!status_code.is_success());
}

