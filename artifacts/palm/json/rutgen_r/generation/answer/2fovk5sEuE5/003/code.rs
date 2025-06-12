// Answer 0

#[test]
fn test_deserialize_any_borrowed() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = &'de str;

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value> {
            Ok(value)
        }

        fn visit_str(self, value: String) -> Result<Self::Value> {
            Err(Error::custom("Should not visit str in this test"))
        }
    }

    struct MockDeserializer {
        de: Deserializer,
    }

    impl MockDeserializer {
        fn new() -> Self {
            MockDeserializer {
                de: Deserializer {
                    read: MockReader::new("test_string"),
                    scratch: String::new(),
                },
            }
        }
    }

    struct MockReader {
        input: &'static str,
        position: usize,
    }

    impl MockReader {
        fn new(input: &'static str) -> Self {
            MockReader { input, position: 0 }
        }
        
        fn parse_str(&mut self, scratch: &mut String) -> Result<Reference> {
            scratch.clear();
            scratch.push_str(self.input);
            Ok(Reference::Borrowed(scratch.as_str()))
        }
    }

    let mut mock_deserializer = MockDeserializer::new();
    let visitor = MockVisitor;
    let result = mock_deserializer.deserialize_any(visitor);
    assert_eq!(result.unwrap(), "test_string");
}

#[test]
#[should_panic]
fn test_deserialize_any_should_panic_on_visit_str() {
    struct PanicVisitor;

    impl<'de> de::Visitor<'de> for PanicVisitor {
        type Value = &'de str;

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value> {
            Err(Error::custom("Should not visit borrowed str in this test"))
        }

        fn visit_str(self, value: String) -> Result<Self::Value> {
            panic!("Panic on visiting str");
        }
    }

    struct PanicDeserializer {
        de: Deserializer,
    }

    impl PanicDeserializer {
        fn new() -> Self {
            PanicDeserializer {
                de: Deserializer {
                    read: PanicReader::new("test_string"),
                    scratch: String::new(),
                },
            }
        }
    }

    struct PanicReader {
        input: &'static str,
        position: usize,
    }

    impl PanicReader {
        fn new(input: &'static str) -> Self {
            PanicReader { input, position: 0 }
        }

        fn parse_str(&mut self, scratch: &mut String) -> Result<Reference> {
            scratch.clear();
            scratch.push_str(self.input);
            Ok(Reference::Copied(scratch.clone()))
        }
    }

    let mut panic_deserializer = PanicDeserializer::new();
    let visitor = PanicVisitor;
    let _ = panic_deserializer.deserialize_any(visitor);
}

