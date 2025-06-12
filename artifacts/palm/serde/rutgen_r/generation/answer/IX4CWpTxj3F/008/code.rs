// Answer 0

#[test]
fn test_fmt_unexpected_seq() {
    use std::fmt;
    use serde::de::Unexpected;

    struct TestUnexpected {
        value: Unexpected,
    }

    let unexpected_seq = TestUnexpected {
        value: Unexpected::Seq,
    };

    let mut formatter = fmt::Formatter::new();
    
    let result = unexpected_seq.value.fmt(&mut formatter);
    
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "sequence");
}

#[test]
fn test_fmt_unexpected_empty_seq() {
    use std::fmt;
    use serde::de::Unexpected;

    struct TestUnexpected {
        value: Unexpected,
    }

    let unexpected_seq = TestUnexpected {
        value: Unexpected::Seq,
    };

    let mut formatter = fmt::Formatter::new();
    
    let result = unexpected_seq.value.fmt(&mut formatter);
    
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "sequence");
}

