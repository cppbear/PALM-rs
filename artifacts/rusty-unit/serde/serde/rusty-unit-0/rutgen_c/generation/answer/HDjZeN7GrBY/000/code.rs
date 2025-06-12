// Answer 0

#[test]
fn test_deserialize_char_with_char() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = char;

        fn visit_char(self, value: char) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(value)
        }

        fn visit_str(self, _value: &str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Expected char".into())
        }

        fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Expected char".into())
        }
    }

    let content = Content::Char('a');
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let result = deserializer.deserialize_char(TestVisitor);
    assert_eq!(result.unwrap(), 'a');
}

#[test]
fn test_deserialize_char_with_string() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_char(self, _value: char) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Expected string".into())
        }

        fn visit_str(self, value: &str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(value.to_string())
        }

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(value.to_string())
        }
    }

    let content = Content::String("hello".to_string());
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let result = deserializer.deserialize_char(TestVisitor);
    assert_eq!(result.unwrap(), "hello");
}

#[test]
fn test_deserialize_char_with_str() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = &'de str;

        fn visit_char(self, _value: char) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Expected str".into())
        }

        fn visit_str(self, value: &str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(value)
        }

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(value)
        }
    }

    let content = Content::Str("world");
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let result = deserializer.deserialize_char(TestVisitor);
    assert_eq!(result.unwrap(), "world");
}

#[test]
#[should_panic]
fn test_deserialize_char_with_invalid_type() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_char(self, _value: char) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(())
        }

        fn visit_str(self, _value: &str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(())
        }

        fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(())
        }
    }

    let content = Content::I32(42);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    deserializer.deserialize_char(TestVisitor).unwrap();
}

