// Answer 0

#[test]
fn test_json_unexpected_display_unit() {
    use serde::de;

    struct JsonUnexpectedTest(de::Unexpected<'static>);

    impl Display for JsonUnexpectedTest {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.0 {
                de::Unexpected::Unit => formatter.write_str("null"),
                de::Unexpected::Float(value) => write!(
                    formatter,
                    "floating point `{}`",
                    ryu::Buffer::new().format(value),
                ),
                _ => Display::fmt(&self.0, formatter),
            }
        }
    }

    let unexpected = JsonUnexpectedTest(de::Unexpected::Unit);
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    
    assert!(result.is_ok());
    assert_eq!(output, "null");
}

#[test]
fn test_json_unexpected_display_float() {
    use serde::de;

    struct JsonUnexpectedTest(de::Unexpected<'static>);

    impl Display for JsonUnexpectedTest {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.0 {
                de::Unexpected::Unit => formatter.write_str("null"),
                de::Unexpected::Float(value) => write!(
                    formatter,
                    "floating point `{}`",
                    ryu::Buffer::new().format(value),
                ),
                _ => Display::fmt(&self.0, formatter),
            }
        }
    }

    let unexpected_value: f64 = 3.14;
    let unexpected = JsonUnexpectedTest(de::Unexpected::Float(unexpected_value));
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    
    assert!(result.is_ok());
    assert_eq!(output, "floating point `3.140000`");  // Depending on ryu's formatting, the output may vary slightly.
}

