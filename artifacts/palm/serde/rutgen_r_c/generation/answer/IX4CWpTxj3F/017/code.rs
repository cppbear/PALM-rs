// Answer 0

#[test]
fn test_fmt_unsigned() {
    use std::fmt;

    #[derive(Copy, Clone, PartialEq, Debug)]
    struct WithDecimalPoint(f64);

    #[derive(Copy, Clone)]
    enum Unexpected<'a> {
        Unsigned(u64),
    }

    impl<'a> fmt::Display for Unexpected<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Unexpected::Unsigned(i) => write!(formatter, "integer `{}`", i),
            }
        }
    }

    let up = Unexpected::Unsigned(42);
    let mut output = String::new();
    let result = write!(&mut output, "{}", up);
    assert!(result.is_ok());
    assert_eq!(output, "integer `42`");
}

#[test]
fn test_fmt_unsigned_zero() {
    use std::fmt;

    #[derive(Copy, Clone, PartialEq, Debug)]
    struct WithDecimalPoint(f64);

    #[derive(Copy, Clone)]
    enum Unexpected<'a> {
        Unsigned(u64),
    }

    impl<'a> fmt::Display for Unexpected<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Unexpected::Unsigned(i) => write!(formatter, "integer `{}`", i),
            }
        }
    }

    let up = Unexpected::Unsigned(0);
    let mut output = String::new();
    let result = write!(&mut output, "{}", up);
    assert!(result.is_ok());
    assert_eq!(output, "integer `0`");
}

#[test]
fn test_fmt_large_unsigned() {
    use std::fmt;

    #[derive(Copy, Clone, PartialEq, Debug)]
    struct WithDecimalPoint(f64);

    #[derive(Copy, Clone)]
    enum Unexpected<'a> {
        Unsigned(u64),
    }

    impl<'a> fmt::Display for Unexpected<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Unexpected::Unsigned(i) => write!(formatter, "integer `{}`", i),
            }
        }
    }

    let up = Unexpected::Unsigned(u64::MAX);
    let mut output = String::new();
    let result = write!(&mut output, "{}", up);
    assert!(result.is_ok());
    assert_eq!(output, format!("integer `{}`", u64::MAX));
}

