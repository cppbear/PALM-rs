// Answer 0

#[test]
fn test_visit_bytes_success() {
    struct StringVisitor {
        value: String,
    }

    impl StringVisitor {
        fn new() -> Self {
            Self {
                value: String::new(),
            }
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<(), E>
        where
            E: serde::de::Error,
        {
            match std::str::from_utf8(v) {
                Ok(s) => {
                    self.value.clear();
                    self.value.push_str(s);
                    Ok(())
                }
                Err(_) => Err(E::invalid_value(serde::de::Unexpected::Bytes(v), &self)),
            }
        }
    }

    let visitor = StringVisitor::new();
    let result = visitor.visit_bytes(b"hello");
    assert!(result.is_ok());
    assert_eq!(visitor.value, "hello");
}

#[test]
#[should_panic]
fn test_visit_bytes_invalid() {
    struct PanickingVisitor {
        value: String,
    }

    impl PanickingVisitor {
        fn new() -> Self {
            Self {
                value: String::new(),
            }
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<(), E>
        where
            E: serde::de::Error,
        {
            match std::str::from_utf8(v) {
                Ok(s) => {
                    self.value.clear();
                    self.value.push_str(s);
                    Ok(())
                }
                Err(_) => Err(E::invalid_value(serde::de::Unexpected::Bytes(v), &self)),
            }
        }
    }

    let visitor = PanickingVisitor::new();
    let _ = visitor.visit_bytes(b"\xFF"); // This will panic due to invalid UTF-8
}

