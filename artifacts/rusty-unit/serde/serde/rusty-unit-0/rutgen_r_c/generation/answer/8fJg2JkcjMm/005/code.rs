// Answer 0

#[test]
fn test_deserialize_any_with_some() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = usize;

        fn visit_bool(mut self, _: bool) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(1)
        }
        
        fn visit_u8(mut self, _: u8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(2)
        }

        fn visit_u16(mut self, _: u16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(3)
        }

        fn visit_u32(mut self, _: u32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(4)
        }

        fn visit_u64(mut self, _: u64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(5)
        }

        fn visit_i8(mut self, _: i8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(6)
        }

        fn visit_i16(mut self, _: i16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(7)
        }

        fn visit_i32(mut self, _: i32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(8)
        }

        fn visit_i64(mut self, _: i64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(9)
        }

        fn visit_f32(mut self, _: f32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(10)
        }

        fn visit_f64(mut self, _: f64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(11)
        }

        fn visit_char(mut self, _: char) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(12)
        }

        fn visit_str(mut self, _: &str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(13)
        }

        fn visit_borrowed_str(mut self, _: &'de str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(14)
        }

        fn visit_bytes(mut self, _: &[u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(15)
        }

        fn visit_borrowed_bytes(mut self, _: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(16)
        }

        fn visit_unit(mut self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(17)
        }

        fn visit_none(mut self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(0)
        }
        
        fn visit_some<V>(mut self, _: V) -> Result<Self::Value, Box<dyn std::error::Error>>
        where
            V: Visitor<'de>,
        {
            Ok(18)
        }
        
        fn visit_newtype_struct<V>(mut self, _: V) -> Result<Self::Value, Box<dyn std::error::Error>>
        where
            V: Visitor<'de>,
        {
            Ok(19)
        }

        fn visit_seq<V>(mut self, _: V) -> Result<Self::Value, Box<dyn std::error::Error>>
        where
            V: Visitor<'de>,
        {
            Ok(20)
        }

        fn visit_map<V>(mut self, _: V) -> Result<Self::Value, Box<dyn std::error::Error>>
        where
            V: Visitor<'de>,
        {
            Ok(21)
        }
    }

    let content_value = Content::Some(Box::new(Content::U8(10)));
    let deserializer = ContentRefDeserializer::new(&content_value);
    let result = deserializer.deserialize_any(VisitorImpl);
    assert_eq!(result.unwrap(), 18);
}

