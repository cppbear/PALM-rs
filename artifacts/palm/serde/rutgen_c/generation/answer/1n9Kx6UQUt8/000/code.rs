// Answer 0

#[test]
fn test_deserialize_any_with_bool_visitor() {
    struct BoolVisitor;
    
    impl<'de> de::Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean")
        }
    }
    
    let bool_deserializer = BoolDeserializer::new(true);
    let result: Result<bool, Box<str>> = bool_deserializer.deserialize_any(BoolVisitor);
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_deserialize_any_with_i32_visitor() {
    struct I32Visitor;

    impl<'de> de::Visitor<'de> for I32Visitor {
        type Value = i32;

        fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i32")
        }
    }
    
    let i32_deserializer = I32Deserializer::new(42);
    let result: Result<i32, Box<str>> = i32_deserializer.deserialize_any(I32Visitor);
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_deserialize_any_with_invalid_length() {
    struct EmptyVisitor;

    impl<'de> de::Visitor<'de> for EmptyVisitor {
        type Value = ();

        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            Err(de::Error::invalid_length(2, &de::Expected))
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of length 2")
        }
    }

    let pair_deserializer = PairDeserializer(BoolDeserializer::new(true), I32Deserializer::new(1), PhantomData);
    let result: Result<(), Box<str>> = pair_deserializer.deserialize_any(EmptyVisitor);
    assert!(result.is_err());
}

