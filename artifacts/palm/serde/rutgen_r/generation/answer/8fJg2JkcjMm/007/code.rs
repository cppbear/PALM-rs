// Answer 0

#[test]
fn test_deserialize_bytes() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_bool<V>(self, _v: bool) -> Result<V::Value, serde::de::Error> {
            unimplemented!()
        }
        
        fn visit_u8<V>(self, _v: u8) -> Result<V::Value, serde::de::Error> {
            unimplemented!()
        }
        
        fn visit_bytes<V>(self, v: &[u8]) -> Result<V::Value, serde::de::Error> {
            Ok(v.to_vec())
        }
        
        fn visit_borrowed_bytes<V>(self, v: &'de [u8]) -> Result<V::Value, serde::de::Error> {
            Ok(v.to_vec())
        }

        // All other visitor methods can be implemented as unimplemented!() for this test.
        fn visit_u16<V>(self, _: u16) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_u32<V>(self, _: u32) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_u64<V>(self, _: u64) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_i8<V>(self, _: i8) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_i16<V>(self, _: i16) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_i32<V>(self, _: i32) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_i64<V>(self, _: i64) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_f32<V>(self, _: f32) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_f64<V>(self, _: f64) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_char<V>(self, _: char) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_str<V>(self, _: &str) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_borrowed_str<V>(self, _: &'de str) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_unit<V>(self) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_none<V>(self) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_some<V>(self, _: ContentRefDeserializer<'de>) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_newtype_struct<V>(self, _: ContentRefDeserializer<'de>) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_seq<V>(self) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_map<V>(self) -> Result<V::Value, serde::de::Error> { unimplemented!() }
    }

    struct Content {
        bytes: Vec<u8>,
    }

    enum ContentEnum {
        Bytes(Vec<u8>),
    }

    struct Deserializer {
        content: ContentEnum,
    }

    impl Deserializer {
        fn new(content: ContentEnum) -> Self {
            Deserializer { content }
        }

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            match self.content {
                ContentEnum::Bytes(ref v) => visitor.visit_bytes(v),
            }
        }
    }

    let deserializer = Deserializer::new(ContentEnum::Bytes(vec![1, 2, 3, 4]));
    let result: Vec<u8> = deserializer.deserialize_any(TestVisitor).unwrap();
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_deserialize_borrowed_bytes() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_bytes<V>(self, v: &[u8]) -> Result<V::Value, serde::de::Error> {
            Ok(v.to_vec())
        }

        fn visit_borrowed_bytes<V>(self, v: &'de [u8]) -> Result<V::Value, serde::de::Error> {
            Ok(v.to_vec())
        }

        // All other visitor methods can be implemented as unimplemented!() for this test.
        fn visit_bool<V>(self, _: bool) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_u8<V>(self, _: u8) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_u16<V>(self, _: u16) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_u32<V>(self, _: u32) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_u64<V>(self, _: u64) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_i8<V>(self, _: i8) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_i16<V>(self, _: i16) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_i32<V>(self, _: i32) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_i64<V>(self, _: i64) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_f32<V>(self, _: f32) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_f64<V>(self, _: f64) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_char<V>(self, _: char) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_str<V>(self, _: &str) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_borrowed_str<V>(self, _: &'de str) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_unit<V>(self) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_none<V>(self) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_some<V>(self, _: ContentRefDeserializer<'de>) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_newtype_struct<V>(self, _: ContentRefDeserializer<'de>) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_seq<V>(self) -> Result<V::Value, serde::de::Error> { unimplemented!() }
        fn visit_map<V>(self) -> Result<V::Value, serde::de::Error> { unimplemented!() }
    }

    struct Content {
        bytes: Vec<u8>,
    }

    enum ContentEnum {
        Bytes(Vec<u8>),
    }

    struct Deserializer {
        content: ContentEnum,
    }

    impl Deserializer {
        fn new(content: ContentEnum) -> Self {
            Deserializer { content }
        }

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            match self.content {
                ContentEnum::Bytes(ref v) => visitor.visit_borrowed_bytes(v),
            }
        }
    }

    let deserializer = Deserializer::new(ContentEnum::Bytes(vec![5, 6, 7, 8]));
    let result: Vec<u8> = deserializer.deserialize_any(TestVisitor).unwrap();
    assert_eq!(result, vec![5, 6, 7, 8]);
}

