// Answer 0

#[test]
fn test_deserialize_empty_string() {
    struct VisitorImpl;

    impl Visitor<'static> for VisitorImpl {
        type Value = String;
        fn visit_borrowed_str(self, v: &'static str) -> Result<Self::Value, serde::de::Error> {
            Ok(v.to_string())
        }
    }

    let deserializer = BorrowedStrDeserializer {
        value: "",
        marker: PhantomData::<serde::de::Error>,
    };
    let visitor = VisitorImpl;

    let _result = deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_single_character_string() {
    struct VisitorImpl;

    impl Visitor<'static> for VisitorImpl {
        type Value = String;
        fn visit_borrowed_str(self, v: &'static str) -> Result<Self::Value, serde::de::Error> {
            Ok(v.to_string())
        }
    }

    let deserializer = BorrowedStrDeserializer {
        value: "a",
        marker: PhantomData::<serde::de::Error>,
    };
    let visitor = VisitorImpl;

    let _result = deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_long_string_with_spaces() {
    struct VisitorImpl;

    impl Visitor<'static> for VisitorImpl {
        type Value = String;
        fn visit_borrowed_str(self, v: &'static str) -> Result<Self::Value, serde::de::Error> {
            Ok(v.to_string())
        }
    }

    let deserializer = BorrowedStrDeserializer {
        value: "long string with spaces",
        marker: PhantomData::<serde::de::Error>,
    };
    let visitor = VisitorImpl;

    let _result = deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_string_with_special_characters() {
    struct VisitorImpl;

    impl Visitor<'static> for VisitorImpl {
        type Value = String;
        fn visit_borrowed_str(self, v: &'static str) -> Result<Self::Value, serde::de::Error> {
            Ok(v.to_string())
        }
    }

    let deserializer = BorrowedStrDeserializer {
        value: "string with special characters !@#$%^&*()",
        marker: PhantomData::<serde::de::Error>,
    };
    let visitor = VisitorImpl;

    let _result = deserializer.deserialize_any(visitor);
}

#[test]
fn test_deserialize_unicode_string() {
    struct VisitorImpl;

    impl Visitor<'static> for VisitorImpl {
        type Value = String;
        fn visit_borrowed_str(self, v: &'static str) -> Result<Self::Value, serde::de::Error> {
            Ok(v.to_string())
        }
    }

    let deserializer = BorrowedStrDeserializer {
        value: "unicode 🔥",
        marker: PhantomData::<serde::de::Error>,
    };
    let visitor = VisitorImpl;

    let _result = deserializer.deserialize_any(visitor);
}

