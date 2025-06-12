// Answer 0

#[test]
fn test_fmt_invalid_status_code() {
    struct InvalidStatus;

    impl std::fmt::Display for InvalidStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("invalid status code")
        }
    }

    let status = InvalidStatus;
    let mut buf = String::new();
    let result = write!(&mut buf, "{}", status);
    
    assert!(result.is_ok());
    assert_eq!(buf, "invalid status code");
}

