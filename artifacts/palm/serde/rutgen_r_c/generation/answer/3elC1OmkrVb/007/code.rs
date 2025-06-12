// Answer 0

#[test]
fn test_deserialize_any_with_bytes() {
    struct MockVisitor {
        called_bytes: bool,
        called_borrowed_bytes: bool,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_bool(self, _: bool) -> Result<Self::Value, ()> {
            panic!("visit_bool should not be called");
        }

        fn visit_u8(self, _: u8) -> Result<Self::Value, ()> {
            panic!("visit_u8 should not be called");
        }

        fn visit_u16(self, _: u16) -> Result<Self::Value, ()> {
            panic!("visit_u16 should not be called");
        }

        fn visit_u32(self, _: u32) -> Result<Self::Value, ()> {
            panic!("visit_u32 should not be called");
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, ()> {
            panic!("visit_u64 should not be called");
        }

        fn visit_i8(self, _: i8) -> Result<Self::Value, ()> {
            panic!("visit_i8 should not be called");
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, ()> {
            panic!("visit_i16 should not be called");
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, ()> {
            panic!("visit_i32 should not be called");
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, ()> {
            panic!("visit_i64 should not be called");
        }

        fn visit_f32(self, _: f32) -> Result<Self::Value, ()> {
            panic!("visit_f32 should not be called");
        }

        fn visit_f64(self, _: f64) -> Result<Self::Value, ()> {
            panic!("visit_f64 should not be called");
        }

        fn visit_char(self, _: char) -> Result<Self::Value, ()> {
            panic!("visit_char should not be called");
        }

        fn visit_string(self, _: String) -> Result<Self::Value, ()> {
            panic!("visit_string should not be called");
        }

        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, ()> {
            panic!("visit_borrowed_str should not be called");
        }

        fn visit_byte_buf(self, _: Vec<u8>) -> Result<Self::Value, ()> {
            self.called_bytes = true;
            Ok(())
        }

        fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value, ()> {
            self.called_borrowed_bytes = true;
            Ok(())
        }

        fn visit_unit(self) -> Result<Self::Value, ()> {
            panic!("visit_unit should not be called");
        }

        fn visit_none(self) -> Result<Self::Value, ()> {
            panic!("visit_none should not be called");
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, ()>
        where
            V: Visitor<'de>,
        {
            panic!("visit_some should not be called");
        }

        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, ()>
        where
            V: Visitor<'de>,
        {
            panic!("visit_newtype_struct should not be called");
        }

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, ()>
        where
            V: Visitor<'de>,
        {
            panic!("visit_seq should not be called");
        }

        fn visit_map<V>(self, _: V) -> Result<Self::Value, ()>
        where
            V: Visitor<'de>,
        {
            panic!("visit_map should not be called");
        }

        fn visit_enum<V>(self, _: V) -> Result<Self::Value, ()>
        where
            V: Visitor<'de>,
        {
            panic!("visit_enum should not be called");
        }

        fn visit_identifier(self, _: &'de str) -> Result<Self::Value, ()> {
            panic!("visit_identifier should not be called");
        }

        fn visit_ignored_any(self) -> Result<Self::Value, ()> {
            panic!("visit_ignored_any should not be called");
        }
    }

    let content = Content::Bytes(vec![1, 2, 3]);
    let deserializer = ContentDeserializer::new(content);
    let mut visitor = MockVisitor {
        called_bytes: false,
        called_borrowed_bytes: false,
    };
    
    let result = deserializer.deserialize_any(visitor);
    
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
    assert!(visitor.called_bytes);
    assert!(!visitor.called_borrowed_bytes);
}

