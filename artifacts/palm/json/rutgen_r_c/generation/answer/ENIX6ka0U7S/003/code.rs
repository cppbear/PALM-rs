// Answer 0

#[test]
fn test_json_unexpected_display_float() {
    use serde::de;

    struct TestUnexpectedFloat {
        unexpected: de::Unexpected<'static>,
    }

    let value: f64 = 3.14159;
    let unexpected_float = TestUnexpectedFloat {
        unexpected: de::Unexpected::Float(value),
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", JsonUnexpected(unexpected_float.unexpected));
    
    assert!(result.is_ok());
    assert_eq!(output, format!("floating point `{}`", ryu::Buffer::new().format(value)));
}

#[test]
fn test_json_unexpected_display_unit() {
    use serde::de;

    struct TestUnexpectedUnit {
        unexpected: de::Unexpected<'static>,
    }

    let unexpected_unit = TestUnexpectedUnit {
        unexpected: de::Unexpected::Unit,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", JsonUnexpected(unexpected_unit.unexpected));
    
    assert!(result.is_ok());
    assert_eq!(output, "null");
}

#[test]
fn test_json_unexpected_display_other() {
    use serde::de;

    struct TestUnexpectedOther {
        unexpected: de::Unexpected<'static>,
    }

    let unexpected_other = TestUnexpectedOther {
        unexpected: de::Unexpected::Str("Unexpected String"),
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", JsonUnexpected(unexpected_other.unexpected));
    
    assert!(result.is_ok());
    assert_eq!(output, "Unexpected String");
}

