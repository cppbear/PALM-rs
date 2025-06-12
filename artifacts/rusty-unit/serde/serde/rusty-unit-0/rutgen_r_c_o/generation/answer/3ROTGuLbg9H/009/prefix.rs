// Answer 0

#[test]
fn test_deserialize_integer_u8_zero() {
    struct TestVisitor;
    impl Visitor<'_> for TestVisitor {
        type Value = u8;

        fn visit_u8(self, value: u8) -> Result<Self::Value, ()> {
            Ok(value)
        }

        // Implement other visit methods as no-ops
        fn visit_u16(self, _: u16) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_u32(self, _: u32) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_u64(self, _: u64) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_i8(self, _: i8) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_i16(self, _: i16) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_i32(self, _: i32) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_i64(self, _: i64) -> Result<Self::Value, ()> { unreachable!() }
    }

    let content = Content::U8(0);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };
    let visitor = TestVisitor;
    let _ = deserializer.deserialize_integer(visitor);
}

#[test]
fn test_deserialize_integer_u8_max() {
    struct TestVisitor;
    impl Visitor<'_> for TestVisitor {
        type Value = u8;

        fn visit_u8(self, value: u8) -> Result<Self::Value, ()> {
            Ok(value)
        }

        // Implement other visit methods as no-ops
        fn visit_u16(self, _: u16) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_u32(self, _: u32) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_u64(self, _: u64) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_i8(self, _: i8) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_i16(self, _: i16) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_i32(self, _: i32) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_i64(self, _: i64) -> Result<Self::Value, ()> { unreachable!() }
    }

    let content = Content::U8(255);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };
    let visitor = TestVisitor;
    let _ = deserializer.deserialize_integer(visitor);
}

#[test]
fn test_deserialize_integer_i8() {
    struct TestVisitor;
    impl Visitor<'_> for TestVisitor {
        type Value = u8;

        fn visit_u8(self, _: u8) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_i8(self, value: i8) -> Result<Self::Value, ()> {
            Ok(value as u8)
        }

        // Implement other visit methods as no-ops
        fn visit_u16(self, _: u16) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_u32(self, _: u32) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_u64(self, _: u64) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_i16(self, _: i16) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_i32(self, _: i32) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_i64(self, _: i64) -> Result<Self::Value, ()> { unreachable!() }
    }

    let content = Content::I8(-1);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };
    let visitor = TestVisitor;
    let _ = deserializer.deserialize_integer(visitor);
}

#[test]
fn test_deserialize_integer_invalid() {
    struct TestVisitor;
    impl Visitor<'_> for TestVisitor {
        type Value = u8;

        fn visit_u8(self, _: u8) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_i8(self, _: i8) -> Result<Self::Value, ()> { unreachable!() }

        // Implement other visit methods as no-ops
        fn visit_u16(self, _: u16) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_u32(self, _: u32) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_u64(self, _: u64) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_i16(self, _: i16) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_i32(self, _: i32) -> Result<Self::Value, ()> { unreachable!() }
        fn visit_i64(self, _: i64) -> Result<Self::Value, ()> { unreachable!() }
    }

    let content = Content::F32(1.0);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };
    let visitor = TestVisitor;
    let _ = deserializer.deserialize_integer(visitor);
}

