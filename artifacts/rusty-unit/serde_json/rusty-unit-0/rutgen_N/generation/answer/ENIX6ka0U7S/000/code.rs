// Answer 0

#[test]
fn test_fmt_unit() {
    use std::fmt::{self, Display};

    struct Unexpected(de::Unexpected);

    impl Display for Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.0 {
                de::Unexpected::Unit => formatter.write_str("null"),
                de::Unexpected::Float(value) => write!(
                    formatter,
                    "floating point `{}`",
                    ryu::Buffer::new().format(value),
                ),
                unexp => Display::fmt(&unexp, formatter),
            }
        }
    }

    let unexpected_unit = Unexpected(de::Unexpected::Unit);
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected_unit.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.into_inner().to_string(), "null");
}

#[test]
fn test_fmt_float() {
    use std::fmt::{self, Display};

    struct Unexpected(de::Unexpected);

    impl Display for Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.0 {
                de::Unexpected::Unit => formatter.write_str("null"),
                de::Unexpected::Float(value) => write!(
                    formatter,
                    "floating point `{}`",
                    ryu::Buffer::new().format(value),
                ),
                unexp => Display::fmt(&unexp, formatter),
            }
        }
    }

    let value = 3.14f64; // Example floating point value
    let unexpected_float = Unexpected(de::Unexpected::Float(value));
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected_float.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.into_inner().to_string(), "floating point `3.14`"); // Ensure formatting is as expected
}

