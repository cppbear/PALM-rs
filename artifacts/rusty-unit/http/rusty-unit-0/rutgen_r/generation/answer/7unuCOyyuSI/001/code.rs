// Answer 0

#[test]
fn test_fmt_invalid_header_value() {
    struct InvalidHeaderValue;

    impl std::fmt::Debug for InvalidHeaderValue {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct("InvalidHeaderValue")
                .finish()
        }
    }

    let value = InvalidHeaderValue;
    let mut output = String::new();
    
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        let result = value.fmt(&mut formatter);
        assert!(result.is_ok());
    }

    assert_eq!(output, "InvalidHeaderValue");
}

#[test]
#[should_panic]
fn test_fmt_invalid_header_value_panic() {
    struct InvalidHeaderValue;

    impl std::fmt::Debug for InvalidHeaderValue {
        fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
            panic!("Intentional panic");
        }
    }

    let value = InvalidHeaderValue;
    
    // This will panic and thus should be caught by the #[should_panic] attribute
    let _ = value.fmt(&mut std::fmt::Formatter::new(&mut String::new()));
}

