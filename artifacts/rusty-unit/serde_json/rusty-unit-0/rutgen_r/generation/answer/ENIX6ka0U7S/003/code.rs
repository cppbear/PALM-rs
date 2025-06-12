// Answer 0

#[test]
fn test_fmt_float() {
    use std::fmt;
    use serde_json::de;

    struct TestUnexpected(de::Unexpected);

    impl fmt::Display for TestUnexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.0 {
                de::Unexpected::Unit => formatter.write_str("null"),
                de::Unexpected::Float(value) => write!(
                    formatter,
                    "floating point `{}`",
                    ryu::Buffer::new().format(value),
                ),
                _ => Err(fmt::Error),
            }
        }
    }

    // Test case for a valid floating-point value
    let float_value: f64 = 3.14;
    let test_unexpected = TestUnexpected(de::Unexpected::Float(float_value));
    
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", test_unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "floating point `3.140000`"); // Expect the formatted output
}

#[test]
fn test_fmt_null() {
    use std::fmt;
    use serde_json::de;

    struct TestUnexpected(de::Unexpected);

    impl fmt::Display for TestUnexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.0 {
                de::Unexpected::Unit => formatter.write_str("null"),
                de::Unexpected::Float(value) => write!(
                    formatter,
                    "floating point `{}`",
                    ryu::Buffer::new().format(value),
                ),
                _ => Err(fmt::Error),
            }
        }
    }

    // Test case for unit variant (null)
    let test_unexpected = TestUnexpected(de::Unexpected::Unit);
    
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", test_unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "null"); // Expect the output to be "null"
}

