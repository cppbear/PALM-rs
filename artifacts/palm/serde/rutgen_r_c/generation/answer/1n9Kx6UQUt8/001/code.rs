// Answer 0

#[test]
fn test_deserialize_any_with_valid_visitor() {
    use std::marker::PhantomData;

    struct TestVisitor {
        value: Option<(bool, i32)>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = (bool, i32);

        fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            let first: bool = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &de::Expected))?;
            let second: i32 = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &de::Expected))?;
            Ok((first, second))
        }

        // a requirement for the visitor, but we do not need to implement it as it won't be called
        fn end(self) -> Result<Self::Value, Self::Error> {
            unimplemented!()
        }
    }

    let deserializer = PairDeserializer::<BoolDeserializer, I32Deserializer, ()>(
        BoolDeserializer::new(true),
        I32Deserializer::new(42),
        PhantomData,
    );

    let visitor = TestVisitor { value: None };
    let result: Result<(bool, i32), ()> = deserializer.deserialize_any(visitor);

    assert_eq!(result, Ok((true, 42)));
}

#[test]
fn test_deserialize_any_with_invalid_length() {
    use std::marker::PhantomData;

    struct TestVisitor {
        value: Option<(bool, i32)>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = (bool, i32);

        fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            // Only consume one element to trigger the invalid length condition
            let first: bool = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &de::Expected))?;
            Ok((first, 0))  // Artificially setting second value to 0
        }

        // a requirement for the visitor, but we do not need to implement it as it won't be called
        fn end(self) -> Result<Self::Value, Self::Error> {
            unimplemented!()
        }
    }

    let deserializer = PairDeserializer::<BoolDeserializer, I32Deserializer, ()>(
        BoolDeserializer::new(true),
        I32Deserializer::new(42),
        PhantomData,
    );

    let visitor = TestVisitor { value: None };
    let result: Result<(bool, i32), ()> = deserializer.deserialize_any(visitor);

    assert!(result.is_err());
}

#[test]
#[should_panic(expected = "not supported")]
fn test_deserialize_any_panics_on_i128() {
    use std::marker::PhantomData;

    struct InvalidVisitor;

    impl<'de> de::Visitor<'de> for InvalidVisitor {
        type Value = ();

        fn visit_seq<S>(self, _seq: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            unimplemented!() // To trigger an unsupported operation
        }

        // a requirement for the visitor, but we do not need to implement it as it won't be called
        fn end(self) -> Result<Self::Value, Self::Error> {
            unimplemented!()
        }
    }

    let deserializer = PairDeserializer::<I128Deserializer, I128Deserializer, ()>(
        I128Deserializer::new(0),
        I128Deserializer::new(0),
        PhantomData,
    );

    let visitor = InvalidVisitor;
    let _ = deserializer.deserialize_any(visitor);
}

