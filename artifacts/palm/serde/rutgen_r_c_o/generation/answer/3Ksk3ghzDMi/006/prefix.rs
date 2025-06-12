// Answer 0

#[test]
fn test_deserialize_integer_u64_min() {
    struct MockVisitor;

    impl Visitor<'static> for MockVisitor {
        type Value = ();

        fn visit_u8(self, _value: u8) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u16(self, _value: u16) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u32(self, _value: u32) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u64(self, _value: u64) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i8(self, _value: i8) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i16(self, _value: i16) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i32(self, _value: i32) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i64(self, _value: i64) -> Result<Self::Value, ()> { Ok(()) }
    }

    let content = Content::U64(0);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };

    let _ = deserializer.deserialize_integer(MockVisitor);
}

#[test]
fn test_deserialize_integer_u64_max() {
    struct MockVisitor;

    impl Visitor<'static> for MockVisitor {
        type Value = ();

        fn visit_u8(self, _value: u8) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u16(self, _value: u16) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u32(self, _value: u32) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u64(self, _value: u64) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i8(self, _value: i8) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i16(self, _value: i16) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i32(self, _value: i32) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i64(self, _value: i64) -> Result<Self::Value, ()> { Ok(()) }
    }

    let content = Content::U64(18446744073709551615);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };

    let _ = deserializer.deserialize_integer(MockVisitor);
}

#[test]
fn test_deserialize_integer_u64_middle() {
    struct MockVisitor;

    impl Visitor<'static> for MockVisitor {
        type Value = ();

        fn visit_u8(self, _value: u8) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u16(self, _value: u16) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u32(self, _value: u32) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_u64(self, _value: u64) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i8(self, _value: i8) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i16(self, _value: i16) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i32(self, _value: i32) -> Result<Self::Value, ()> { Ok(()) }
        fn visit_i64(self, _value: i64) -> Result<Self::Value, ()> { Ok(()) }
    }

    let content = Content::U64(9223372036854775807);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };

    let _ = deserializer.deserialize_integer(MockVisitor);
}

