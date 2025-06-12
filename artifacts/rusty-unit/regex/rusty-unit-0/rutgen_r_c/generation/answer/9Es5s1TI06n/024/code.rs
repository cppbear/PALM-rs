// Answer 0

#[test]
fn test_decimal_invalid_display() {
    use std::fmt::Write;

    #[derive(Clone, Copy, Eq, PartialEq)]
    struct DummySpan;

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum DummyErrorKind {
        DecimalInvalid,
    }

    impl fmt::Display for DummyErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                DummyErrorKind::DecimalInvalid => write!(f, "decimal literal invalid"),
            }
        }
    }

    let error_kind = DummyErrorKind::DecimalInvalid;
    let mut output = String::new();
    assert_eq!(error_kind.to_string(), "decimal literal invalid");
}

