// Answer 0

#[test]
fn test_deserialize_integer_i8_valid() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i8;
        fn visit_i8(self, value: i8) -> Result<Self::Value, E> {
            Ok(value)
        }
        // Implement other visitor methods as no-op if needed.
    }

    let content = Content::I8(100);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };
    let _ = deserializer.deserialize_integer(TestVisitor);
}

#[test]
fn test_deserialize_integer_i8_boundary_min() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i8;
        fn visit_i8(self, value: i8) -> Result<Self::Value, E> {
            Ok(value)
        }
        // Implement other visitor methods as no-op if needed.
    }

    let content = Content::I8(-128);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };
    let _ = deserializer.deserialize_integer(TestVisitor);
}

#[test]
fn test_deserialize_integer_i8_boundary_max() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i8;
        fn visit_i8(self, value: i8) -> Result<Self::Value, E> {
            Ok(value)
        }
        // Implement other visitor methods as no-op if needed.
    }

    let content = Content::I8(127);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };
    let _ = deserializer.deserialize_integer(TestVisitor);
}

#[test]
#[should_panic]
fn test_deserialize_integer_i8_invalid() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i8;
        fn visit_i8(self, value: i8) -> Result<Self::Value, E> {
            Ok(value)
        }
        // Implement other visitor methods as no-op if needed.
    }

    let content = Content::U8(255); // Invalid type for I8
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };
    let _ = deserializer.deserialize_integer(TestVisitor);
}

