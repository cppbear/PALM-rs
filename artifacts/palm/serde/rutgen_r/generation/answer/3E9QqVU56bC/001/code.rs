// Answer 0

#[test]
fn test_deserialize_i32_valid() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i32")
        }

        fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> {
            Ok(value)
        }
        
        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
            if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                Ok(value as i32)
            } else {
                Err(serde::de::Error::custom("value out of range for i32"))
            }
        }
    }

    struct TestDeserializer;

    impl serde::de::Deserializer<'_> for TestDeserializer {
        type Error = serde::de::value::Error;

        fn deserialize_integer<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'_>,
        {
            visitor.visit_i32(42)
        }

        fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserialize_integer(visitor)
        }

        // Other methods are not required for this test
    }

    let deserializer = TestDeserializer;
    let visitor = TestVisitor;

    let result: Result<i32, _> = deserializer.deserialize_i32(visitor);
    assert_eq!(result, Ok(42));
}

#[test]
#[should_panic(expected = "value out of range for i32")]
fn test_deserialize_i32_out_of_range() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i32")
        }
        
        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
            if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                Ok(value as i32)
            } else {
                Err(serde::de::Error::custom("value out of range for i32"))
            }
        }
    }

    struct TestDeserializer;

    impl serde::de::Deserializer<'_> for TestDeserializer {
        type Error = serde::de::value::Error;

        fn deserialize_integer<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'_>,
        {
            visitor.visit_i64(50000) // Out of i32 range
        }

        fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserialize_integer(visitor)
        }
    }

    let deserializer = TestDeserializer;
    let visitor = TestVisitor;

    let _result: Result<i32, _> = deserializer.deserialize_i32(visitor);
}

