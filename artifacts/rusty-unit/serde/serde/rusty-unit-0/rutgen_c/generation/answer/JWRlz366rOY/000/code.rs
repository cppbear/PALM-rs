// Answer 0

#[test]
fn test_content_deserializer_new_with_bool() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean")
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
            Ok(v)
        }

        // ... other methods required by Visitor can be ignored for this test case
    }

    let content = Content::Bool(true);
    let deserializer = ContentDeserializer::new(content);
    let result: bool = deserializer.deserialize_any(TestVisitor).unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_content_deserializer_new_with_integer() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an integer")
        }

        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> {
            Ok(v)
        }

        // ... other methods required by Visitor can be ignored for this test case
    }

    let content = Content::I32(42);
    let deserializer = ContentDeserializer::new(content);
    let result: i32 = deserializer.deserialize_any(TestVisitor).unwrap();
    assert_eq!(result, 42);
}

#[should_panic(expected = "i128 is not supported")]
#[test]
fn test_content_deserializer_new_with_i128() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i128;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i128")
        }

        fn visit_i128<E>(self, _v: i128) -> Result<Self::Value, E> {
            panic!("This should not succeed")
        }

        // ... other methods required by Visitor can be ignored for this test case
    }

    let content = Content::I128(12345678901234567890);
    let deserializer = ContentDeserializer::new(content);
    deserializer.deserialize_i128(TestVisitor).unwrap();
}

#[test]
fn test_content_deserializer_new_with_option() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an Option integer")
        }

        fn visit_some<E>(self, value: i32) -> Result<Self::Value, E> {
            Ok(Some(value))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        // ... other methods required by Visitor can be ignored for this test case
    }

    let content = Content::Some(Box::new(Content::I32(5)));
    let deserializer = ContentDeserializer::new(content);
    let result: Option<i32> = deserializer.deserialize_option(TestVisitor).unwrap();
    assert_eq!(result, Some(5));
}

#[test]
fn test_content_deserializer_new_with_string() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
            Ok(v)
        }

        // ... other methods required by Visitor can be ignored for this test case
    }

    let content = Content::String("hello".to_string());
    let deserializer = ContentDeserializer::new(content);
    let result: String = deserializer.deserialize_string(TestVisitor).unwrap();
    assert_eq!(result, "hello");
}

