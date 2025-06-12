// Answer 0

#[test]
fn test_fmt_unit() {
    use std::fmt::{self, Display};
    
    struct Unexpected(de::Unexpected);
    
    impl Display for Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.0 {
                de::Unexpected::Unit => formatter.write_str("null"),
                _ => formatter.write_str("unexpected value"),
            }
        }
    }
    
    let unexpected = Unexpected(de::Unexpected::Unit);
    let mut output = String::new();
    let result = unexpected.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "null");
}

#[test]
fn test_fmt_float() {
    use std::fmt::{self, Display};
    
    struct Unexpected(de::Unexpected);
    
    impl Display for Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.0 {
                de::Unexpected::Float(value) => write!(
                    formatter,
                    "floating point `{}`",
                    ryu::Buffer::new().format(value),
                ),
                _ => formatter.write_str("unexpected value"),
            }
        }
    }
    
    let value: f64 = 3.14;
    let unexpected = Unexpected(de::Unexpected::Float(value));
    let mut output = String::new();
    let result = unexpected.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "floating point `3.14`");
}

