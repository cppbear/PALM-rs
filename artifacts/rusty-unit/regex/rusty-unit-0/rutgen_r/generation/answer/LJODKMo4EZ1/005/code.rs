// Answer 0

#[test]
fn test_description_invalid_utf8() {
    struct ErrorKind {
        kind: String,
    }

    impl ErrorKind {
        const InvalidUtf8: ErrorKind = ErrorKind { kind: "InvalidUtf8".to_string() };
        
        fn description(&self) -> &str {
            match self.kind.as_str() {
                "UnicodeNotAllowed" => "Unicode not allowed here",
                "InvalidUtf8" => "pattern can match invalid UTF-8",
                "UnicodePropertyNotFound" => "Unicode property not found",
                "UnicodePropertyValueNotFound" => "Unicode property value not found",
                "EmptyClassNotAllowed" => "empty character classes are not allowed",
                _ => unreachable!(),
            }
        }
    }

    let error = ErrorKind::InvalidUtf8;
    assert_eq!(error.description(), "pattern can match invalid UTF-8");
}

