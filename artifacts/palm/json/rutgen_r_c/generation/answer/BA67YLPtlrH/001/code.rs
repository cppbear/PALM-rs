// Answer 0

#[test]
fn test_deserialize_any_owned_string() {
    use alloc::borrow::Cow;
    use serde::de::Visitor;

    struct DummyVisitor {
        visited: bool,
    }

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = String;

        fn visit_borrowed_str(self, _value: &'de str) -> Result<String, Error> {
            Err(Error {})
        }

        fn visit_string(self, value: String) -> Result<String, Error> {
            self.visited = true;
            Ok(value)
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string")
        }
    }

    let owned_string = Cow::Owned(String::from("test owned string"));
    let deserializer = BorrowedCowStrDeserializer { value: owned_string };
    let visitor = DummyVisitor { visited: false };
    
    let result = deserializer.deserialize_any(visitor);

    assert!(result.is_ok());
    assert!(result.unwrap() == "test owned string");
}

#[test]
fn test_deserialize_any_borrowed_string() {
    use alloc::borrow::Cow;
    use serde::de::Visitor;

    struct DummyVisitor {
        visited: bool,
    }

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = String;

        fn visit_borrowed_str(self, value: &'de str) -> Result<String, Error> {
            self.visited = true;
            Ok(value.to_owned())
        }

        fn visit_string(self, _value: String) -> Result<String, Error> {
            Err(Error {})
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string")
        }
    }

    let borrowed_string = Cow::Borrowed("test borrowed string");
    let deserializer = BorrowedCowStrDeserializer { value: borrowed_string };
    let visitor = DummyVisitor { visited: false };
    
    let result = deserializer.deserialize_any(visitor);

    assert!(result.is_ok());
    assert!(result.unwrap() == "test borrowed string");
}

