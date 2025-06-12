// Answer 0

#[test]
fn test_fmt_valid() {
    struct ValidRegex;
    impl std::fmt::Display for ValidRegex {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "valid_regex")
        }
    }

    let regex = ValidRegex;
    let mut output = String::new();
    let result = regex.fmt(&mut std::fmt::Formatter::new(&mut output));

    assert!(result.is_ok());
    assert_eq!(output, "valid_regex");
}

#[test]
#[should_panic]
fn test_fmt_panic() {
    struct PanicRegex;
    impl std::fmt::Display for PanicRegex {
        fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
            panic!("This should panic");
        }
    }

    let regex = PanicRegex;
    let _ = regex.fmt(&mut std::fmt::Formatter::new(&mut String::new()));
}

#[test]
fn test_fmt_empty() {
    struct EmptyRegex;
    impl std::fmt::Display for EmptyRegex {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "")
        }
    }

    let regex = EmptyRegex;
    let mut output = String::new();
    let result = regex.fmt(&mut std::fmt::Formatter::new(&mut output));

    assert!(result.is_ok());
    assert_eq!(output, "");
}

