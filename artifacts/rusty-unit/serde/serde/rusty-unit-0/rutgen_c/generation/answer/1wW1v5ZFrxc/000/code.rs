// Answer 0

#[test]
fn test_deserialize_any_with_borrowed_str() {
    struct MockVisitor {
        value: &'static str,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = &'de str;

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, ()> {
            assert_eq!(value, self.value);
            Ok(value)
        }

        // Implement other required methods with no-op if necessary
        fn visit_enum<V>(self) -> Result<(Self::Value, V), ()> where V: de::VariantAccess<'de, Error = ()> {
            Err(())
        }
    }

    let value = "test";
    let deserializer = BorrowedStrDeserializer {
        value,
        marker: PhantomData,
    };
    let visitor = MockVisitor { value };

    let result: Result<_, ()> = deserializer.deserialize_any(visitor);
    assert_eq!(result, Ok(value));
}

#[test]
#[should_panic]
fn test_deserialize_any_with_different_borrowed_str() {
    struct MockVisitor {
        value: &'static str,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = &'de str;

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, ()> {
            assert_eq!(value, self.value);
            Ok(value)
        }

        // Implement other required methods with no-op if necessary
        fn visit_enum<V>(self) -> Result<(Self::Value, V), ()> where V: de::VariantAccess<'de, Error = ()> {
            Err(())
        }
    }

    let value = "test";
    let deserializer = BorrowedStrDeserializer {
        value,
        marker: PhantomData,
    };
    let visitor = MockVisitor { value: "different" };

    deserializer.deserialize_any(visitor).unwrap();
}

