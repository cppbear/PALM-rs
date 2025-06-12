// Answer 0

#[test]
fn test_deserialize_borrowed_str() {
    struct DummyVisitor;

    impl<'de> de::Visitor<'de> for DummyVisitor {
        type Value = &'de str;

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
        
        fn visit_enum<V>(self) -> Result<(V, V::Variant), Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(de::Error::custom("Not implemented"))
        }
    }

    let value = "test";
    let deserializer: BorrowedStrDeserializer<DummyError> = BorrowedStrDeserializer {
        value,
        marker: PhantomData,
    };

    let result: Result<&str, DummyError> = deserializer.deserialize_any(DummyVisitor);

    assert_eq!(result.unwrap(), "test");
}

#[test]
fn test_deserialize_borrowed_str_empty() {
    struct DummyVisitor;

    impl<'de> de::Visitor<'de> for DummyVisitor {
        type Value = &'de str;

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
        
        fn visit_enum<V>(self) -> Result<(V, V::Variant), Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(de::Error::custom("Not implemented"))
        }
    }

    let value = "";
    let deserializer: BorrowedStrDeserializer<DummyError> = BorrowedStrDeserializer {
        value,
        marker: PhantomData,
    };

    let result: Result<&str, DummyError> = deserializer.deserialize_any(DummyVisitor);

    assert_eq!(result.unwrap(), "");
}

#[test]
#[should_panic(expected = "custom error")]
fn test_deserialize_borrowed_str_panic() {
    struct PanicVisitor;

    impl<'de> de::Visitor<'de> for PanicVisitor {
        type Value = &'de str;

        fn visit_borrowed_str<E>(self, _: &'de str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            panic!("custom error")
        }

        fn visit_enum<V>(self) -> Result<(V, V::Variant), Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(de::Error::custom("Not implemented"))
        }
    }

    let value = "will panic";
    let deserializer: BorrowedStrDeserializer<DummyError> = BorrowedStrDeserializer {
        value,
        marker: PhantomData,
    };

    let _ = deserializer.deserialize_any(PanicVisitor);
}

struct DummyError;

impl de::Error for DummyError {
    fn custom<T: std::fmt::Display>(_msg: T) -> Self {
        DummyError
    }
    fn invalid_value(_msg: &str) -> Self {
        DummyError
    }
    // Implement further methods as necessary
}

