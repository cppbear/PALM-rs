// Answer 0

#[test]
fn test_from_str_valid_standard_scheme() {
    struct TestScheme;
    impl FromStr for TestScheme {
        type Err = InvalidUri;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Scheme::from_str(s).map(|_| TestScheme)
        }
    }

    let scheme_str = "http";
    let result = TestScheme::from_str(scheme_str);
    assert!(result.is_ok());
}

#[test]
fn test_from_str_invalid_scheme() {
    struct TestScheme;
    impl FromStr for TestScheme {
        type Err = InvalidUri;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Scheme::from_str(s).map(|_| TestScheme)
        }
    }

    let scheme_str = "invalid_scheme_with_special_chars_#";
    let result = TestScheme::from_str(scheme_str);
    assert!(result.is_err());
}

#[test]
fn test_from_str_exceeds_max_length() {
    struct TestScheme;
    impl FromStr for TestScheme {
        type Err = InvalidUri;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Scheme::from_str(s).map(|_| TestScheme)
        }
    }

    let scheme_str = "http://this/is/a/very/long/scheme/name/that/exceeds/the/maximum/length/allowed/in/a/uri/scheme";
    let result = TestScheme::from_str(scheme_str);
    assert!(result.is_err());
}

#[test]
fn test_from_str_valid_other_scheme() {
    struct TestScheme;
    impl FromStr for TestScheme {
        type Err = InvalidUri;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Scheme::from_str(s).map(|_| TestScheme)
        }
    }

    let scheme_str = "ftp";
    let result = TestScheme::from_str(scheme_str);
    assert!(result.is_ok());
}

