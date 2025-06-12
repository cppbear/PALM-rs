// Answer 0

fn test_deserialize_integer_invalid_type() {
    struct MockVisitor {
        value: Result<(), &'static str>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_u8(self, _: u8) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_u8")
        }

        fn visit_u16(self, _: u16) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_u16")
        }

        fn visit_u32(self, _: u32) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_u32")
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_u64")
        }

        fn visit_i8(self, _: i8) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_i8")
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_i16")
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_i32")
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_i64")
        }

        fn visit_f32(self, _: f32) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_f32")
        }

        fn visit_f64(self, _: f64) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_f64")
        }

        fn visit_char(self, _: char) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_char")
        }

        fn visit_str(self, _: &str) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_str")
        }

        fn visit_string(self, _: String) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_string")
        }

        fn visit_bytes(self, _: &[u8]) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_bytes")
        }

        fn visit_byte_buf(self, _: Vec<u8>) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_byte_buf")
        }

        fn visit_none(self) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_none")
        }

        fn visit_some<D>(self, _: D) -> Result<Self::Value, &'static str>
        where
            D: Deserializer<'de>,
        {
            Err("unexpected visit_some")
        }

        fn visit_unit(self) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_unit")
        }

        fn visit_unit_struct(self, _: &'static str) -> Result<Self::Value, &'static str> {
            Err("unexpected visit_unit_struct")
        }

        fn visit_newtype_struct<D>(self, _: &'static str, _: D) -> Result<Self::Value, &'static str>
        where
            D: Deserializer<'de>,
        {
            Err("unexpected visit_newtype_struct")
        }

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, &'static str>
        where
            V: SeqAccess<'de>,
        {
            Err("unexpected visit_seq")
        }

        fn visit_map<V>(self, _: V) -> Result<Self::Value, &'static str>
        where
            V: MapAccess<'de>,
        {
            Err("unexpected visit_map")
        }
    }

    let content = Content::Str("invalid type");
    let deserializer = ContentDeserializer { content, err: PhantomData::<&'static str> };

    let result = deserializer.deserialize_integer(MockVisitor { value: Ok(()) });

    assert!(result.is_err());
}

