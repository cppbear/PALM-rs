// Answer 0

#[test]
fn test_is_redirection_with_redirection_codes() {
    struct TestStatusCode(u16);

    impl TestStatusCode {
        fn is_redirection(&self) -> bool {
            (300..400).contains(&self.0)
        }
    }

    let redirection_codes = [300, 301, 302, 303, 304, 305, 307, 308];
    for &code in &redirection_codes {
        let status_code = TestStatusCode(code);
        assert!(status_code.is_redirection(), "Failed for redirection code: {}", code);
    }
}

#[test]
fn test_is_redirection_with_non_redirection_codes() {
    struct TestStatusCode(u16);

    impl TestStatusCode {
        fn is_redirection(&self) -> bool {
            (300..400).contains(&self.0)
        }
    }

    let non_redirection_codes = [200, 201, 400, 404, 500];
    for &code in &non_redirection_codes {
        let status_code = TestStatusCode(code);
        assert!(!status_code.is_redirection(), "Should not be a redirection for code: {}", code);
    }
}

#[test]
fn test_is_redirection_with_boundary_values() {
    struct TestStatusCode(u16);

    impl TestStatusCode {
        fn is_redirection(&self) -> bool {
            (300..400).contains(&self.0)
        }
    }

    let boundary_codes = [299, 300, 399, 400];
    for &code in &boundary_codes {
        let status_code = TestStatusCode(code);
        if code == 300 || code == 399 {
            assert!(status_code.is_redirection(), "Failed for boundary code: {}", code);
        } else {
            assert!(!status_code.is_redirection(), "Should not be a redirection for boundary code: {}", code);
        }
    }
}

