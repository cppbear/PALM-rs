// Answer 0

#[test]
fn test_fmt_signed() {
    use std::fmt;
    
    #[derive(Copy, Clone, PartialEq, Debug)]
    struct WithDecimalPoint(f64);
    
    #[derive(Copy, Clone, PartialEq, Debug)]
    enum Unexpected<'a> {
        Signed(i64),
    }
    
    impl<'a> fmt::Display for Unexpected<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Unexpected::Signed(i) => write!(formatter, "integer `{}`", i),
            }
        }
    }

    let unexpected_signed = Unexpected::Signed(-42);
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected_signed);
    
    assert!(result.is_ok());
    assert_eq!(output, "integer `-42`");
}

#[test]
fn test_fmt_signed_zero() {
    use std::fmt;

    #[derive(Copy, Clone, PartialEq, Debug)]
    struct WithDecimalPoint(f64);

    #[derive(Copy, Clone, PartialEq, Debug)]
    enum Unexpected<'a> {
        Signed(i64),
    }

    impl<'a> fmt::Display for Unexpected<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Unexpected::Signed(i) => write!(formatter, "integer `{}`", i),
            }
        }
    }

    let unexpected_signed_zero = Unexpected::Signed(0);
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected_signed_zero);
    
    assert!(result.is_ok());
    assert_eq!(output, "integer `0`");
}

#[test]
fn test_fmt_signed_large_value() {
    use std::fmt;

    #[derive(Copy, Clone, PartialEq, Debug)]
    struct WithDecimalPoint(f64);

    #[derive(Copy, Clone, PartialEq, Debug)]
    enum Unexpected<'a> {
        Signed(i64),
    }

    impl<'a> fmt::Display for Unexpected<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Unexpected::Signed(i) => write!(formatter, "integer `{}`", i),
            }
        }
    }

    let unexpected_signed_large = Unexpected::Signed(1_000_000);
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected_signed_large);
    
    assert!(result.is_ok());
    assert_eq!(output, "integer `1000000`");
}

