// Answer 0

#[test]
fn test_deserialize_any_string_content() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_bool(self, _: bool) -> Result<Self::Value, serde::de::Error> {
            unreachable!()
        }
        fn visit_u8(self, _: u8) -> Result<Self::Value, serde::de::Error> {
            unreachable!()
        }
        fn visit_u16(self, _: u16) -> Result<Self::Value, serde::de::Error> {
            unreachable!()
        }
        fn visit_u32(self, _: u32) -> Result<Self::Value, serde::de::Error> {
            unreachable!()
        }
        fn visit_u64(self, _: u64) -> Result<Self::Value, serde::de::Error> {
            unreachable!()
        }
        fn visit_i8(self, _: i8) -> Result<Self::Value, serde::de::Error> {
            unreachable!()
        }
        fn visit_i16(self, _: i16) -> Result<Self::Value, serde::de::Error> {
            unreachable!()
        }
        fn visit_i32(self, _: i32) -> Result<Self::Value, serde::de::Error> {
            unreachable!()
        }
        fn visit_i64(self, _: i64) -> Result<Self::Value, serde::de::Error> {
            unreachable!()
        }
        fn visit_f32(self, _: f32) -> Result<Self::Value, serde::de::Error> {
            unreachable!()
        }
        fn visit_f64(self, _: f64) -> Result<Self::Value, serde::de::Error> {
            unreachable!()
        }
        fn visit_char(self, _: char) -> Result<Self::Value, serde::de::Error> {
            unreachable!()
        }
        fn visit_str(self, value: &str) -> Result<Self::Value, serde::de::Error> {
            Ok(value.to_string())
        }
        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, serde::de::Error> {
            Ok(value.to_string())
        }
        fn visit_bytes(self, _: &[u8]) -> Result<Self::Value, serde::de::Error> {
            unreachable!()
        }
        fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value, serde::de::Error> {
            unreachable!()
        }
        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            unreachable!()
        }
        fn visit_none(self) -> Result<Self::Value, serde::de::Error> {
            unreachable!()
        }
        fn visit_some<V>(self, _: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::Deserialize<'de>,
        {
            unreachable!()
        }
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::Deserialize<'de>,
        {
            unreachable!()
        }
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            unreachable!()
        }
        fn visit_map<V>(self, _: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            unreachable!()
        }
    }

    struct Content {
        content: ContentEnum,
    }

    enum ContentEnum {
        String(String),
    }

    impl Content {
        fn deserialize_any<V>(&self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            match &self.content {
                ContentEnum::String(v) => visitor.visit_str(v),
            }
        }
    }

    let content = Content {
        content: ContentEnum::String("Hello, world!".to_string()),
    };

    let result: String = content.deserialize_any(TestVisitor).unwrap();
    assert_eq!(result, "Hello, world!");
}

