// Answer 0

#[test]
fn test_deserialize_any_borrowed() {
    use std::borrow::Cow;
    use serde_json::de::{self, Visitor};
    use serde_json::Error;

    struct TestVisitor<'de> {
        value: Option<&'de str>,
    }

    impl<'de> Visitor<'de> for TestVisitor<'de> {
        type Value = &'de str;

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, Error> {
            Ok(value)
        }

        #[cfg(any(feature = "std", feature = "alloc"))]
        fn visit_string(self, value: String) -> Result<Self::Value, Error> {
            Err(Error::custom("Not used in this test"))
        }
    }

    let value = Cow::Borrowed("test");
    let result = value.deserialize_any(TestVisitor { value: None }).unwrap();
    assert_eq!(result, "test");
}

#[test]
#[cfg(any(feature = "std", feature = "alloc"))]
fn test_deserialize_any_owned() {
    use std::borrow::Cow;
    use serde_json::de::{self, Visitor};
    use serde_json::Error;

    struct TestVisitor<'de> {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for TestVisitor<'de> {
        type Value = String;

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, Error> {
            Err(Error::custom("Not used in this test"))
        }

        fn visit_string(self, value: String) -> Result<Self::Value, Error> {
            Ok(value)
        }
    }

    let value = Cow::Owned("test".to_string());
    let result = value.deserialize_any(TestVisitor { value: None }).unwrap();
    assert_eq!(result, "test");
}

