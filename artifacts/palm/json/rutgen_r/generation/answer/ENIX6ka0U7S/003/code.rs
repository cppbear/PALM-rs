// Answer 0

#[test]
fn test_fmt_float() {
    use std::fmt;
    use serde_json::de;

    struct UnexpectedFloat(de::Unexpected);

    impl fmt::Display for UnexpectedFloat {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.0 {
                de::Unexpected::Unit => formatter.write_str("null"),
                de::Unexpected::Float(value) => write!(
                    formatter,
                    "floating point `{}`",
                    ryu::Buffer::new().format(value),
                ),
                _ => write!(formatter, "other unexpected value"),
            }
        }
    }

    let float_value: f64 = 3.14;
    let unexpected = UnexpectedFloat(de::Unexpected::Float(float_value));
    
    let mut buffer = String::new();
    let result = unexpected.fmt(&mut fmt::Formatter::new());
    assert!(result.is_ok());
    assert_eq!(buffer, "floating point `3.14`");
}

#[test]
fn test_fmt_unit() {
    use std::fmt;
    use serde_json::de;

    struct UnexpectedUnit(de::Unexpected);

    impl fmt::Display for UnexpectedUnit {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.0 {
                de::Unexpected::Unit => formatter.write_str("null"),
                _ => write!(formatter, "other unexpected value"),
            }
        }
    }

    let unexpected = UnexpectedUnit(de::Unexpected::Unit);
    
    let mut buffer = String::new();
    let result = unexpected.fmt(&mut fmt::Formatter::new());
    assert!(result.is_ok());
    assert_eq!(buffer, "null");
}

