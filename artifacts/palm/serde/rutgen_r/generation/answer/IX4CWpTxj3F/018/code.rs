// Answer 0

#[test]
fn test_fmt_bool_true() {
    use std::fmt;

    struct Unexpected {
        inner: Bool,
    }

    struct Bool(bool);

    impl fmt::Display for Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.inner {
                Bool(b) => write!(formatter, "boolean `{}`", b),
            }
        }
    }

    let unexpected_true = Unexpected { inner: Bool(true) };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected_true);
    assert!(result.is_ok());
    assert_eq!(buffer, "boolean `true\"");
}

#[test]
fn test_fmt_bool_false() {
    use std::fmt;

    struct Unexpected {
        inner: Bool,
    }

    struct Bool(bool);

    impl fmt::Display for Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.inner {
                Bool(b) => write!(formatter, "boolean `{}`", b),
            }
        }
    }

    let unexpected_false = Unexpected { inner: Bool(false) };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected_false);
    assert!(result.is_ok());
    assert_eq!(buffer, "boolean `false\"");
}

