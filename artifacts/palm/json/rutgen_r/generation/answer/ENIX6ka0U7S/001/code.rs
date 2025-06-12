// Answer 0

#[test]
fn test_fmt_unexpected() {
    use serde_json::Error;
    use std::fmt;

    struct UnexpectedWrapper(de::Unexpected);

    // Here we create an Unexpected variant that is neither Float nor Unit
    struct OtherVariant;

    impl fmt::Display for OtherVariant {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("other variant")
        }
    }

    impl fmt::Debug for OtherVariant {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("Debug representation of other variant")
        }
    }

    let unexpected = UnexpectedWrapper(de::Unexpected::OtherVariant);

    let mut buffer = fmt::Formatter::new();
    let result = unexpected.fmt(&mut buffer);

    assert!(result.is_ok());
    let output = buffer.buffer();
    assert_eq!(output, "other variant");
}

