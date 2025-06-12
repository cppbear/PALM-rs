// Answer 0

#[test]
fn test_invalid_status_code_format() {
    use std::fmt;

    struct InvalidStatusCode;

    impl fmt::Debug for InvalidStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("InvalidStatusCode")
                .finish()
        }
    }

    let status_code = InvalidStatusCode;
    let mut output = String::new();
    let result = fmt::write(&mut output, format_args!("{:?}", status_code));
    
    assert!(result.is_ok());
    assert_eq!(output, "InvalidStatusCode");
}

