// Answer 0

#[test]
fn test_deserialize_any_with_unit() {
    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn visit_bool(self, _: bool) -> Result<(), Self::Error> {
            unreachable!()
        }
        
        fn visit_u8(self, _: u8) -> Result<(), Self::Error> {
            unreachable!()
        }

        fn visit_u16(self, _: u16) -> Result<(), Self::Error> {
            unreachable!()
        }

        fn visit_u32(self, _: u32) -> Result<(), Self::Error> {
            unreachable!()
        }

        fn visit_u64(self, _: u64) -> Result<(), Self::Error> {
            unreachable!()
        }

        fn visit_i8(self, _: i8) -> Result<(), Self::Error> {
            unreachable!()
        }

        fn visit_i16(self, _: i16) -> Result<(), Self::Error> {
            unreachable!()
        }

        fn visit_i32(self, _: i32) -> Result<(), Self::Error> {
            unreachable!()
        }

        fn visit_i64(self, _: i64) -> Result<(), Self::Error> {
            unreachable!()
        }

        fn visit_f32(self, _: f32) -> Result<(), Self::Error> {
            unreachable!()
        }

        fn visit_f64(self, _: f64) -> Result<(), Self::Error> {
            unreachable!()
        }

        fn visit_char(self, _: char) -> Result<(), Self::Error> {
            unreachable!()
        }

        fn visit_string(self, _: String) -> Result<(), Self::Error> {
            unreachable!()
        }

        fn visit_borrowed_str(self, _: &str) -> Result<(), Self::Error> {
            unreachable!()
        }

        fn visit_byte_buf(self, _: Vec<u8>) -> Result<(), Self::Error> {
            unreachable!()
        }

        fn visit_borrowed_bytes(self, _: &[u8]) -> Result<(), Self::Error> {
            unreachable!()
        }

        fn visit_unit(self) -> Result<(), Self::Error> {
            Ok(())
        }

        fn visit_none(self) -> Result<(), Self::Error> {
            unreachable!()
        }

        fn visit_some<V>(self, _: V) -> Result<(), Self::Error>
        where
            V: Visitor<'de>,
        {
            unreachable!()
        }

        fn visit_newtype_struct<V>(self, _: V) -> Result<(), Self::Error>
        where
            V: Visitor<'de>,
        {
            unreachable!()
        }

        fn visit_seq<V>(self, _: V) -> Result<(), Self::Error>
        where
            V: SeqAccess<'de>,
        {
            unreachable!()
        }

        fn visit_map<V>(self, _: V) -> Result<(), Self::Error>
        where
            V: MapAccess<'de>,
        {
            unreachable!()
        }
    }

    let content = Content::Unit;
    let deserializer = ContentDeserializer::new(content);
    let visitor = TestVisitor;
    
    let _ = deserializer.deserialize_any(visitor);
}

