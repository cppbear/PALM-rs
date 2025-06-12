// Answer 0

#[test]
fn test_serialize_char_valid_char() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_str(&self, value: &str) -> Result<()> {
            assert_eq!(value, "a");
            Ok(())
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_char('a');
    assert!(result.is_ok());
}

#[test]
fn test_serialize_char_valid_unicode_char() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_str(&self, value: &str) -> Result<()> {
            assert_eq!(value, "😊");
            Ok(())
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_char('😊');
    assert!(result.is_ok());
}

#[test]
fn test_serialize_char_boundary_case() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_str(&self, value: &str) -> Result<()> {
            assert_eq!(value, "𐍈"); // U+10380 (gothic letter hwair)
            Ok(())
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_char('𐍈');
    assert!(result.is_ok());
}

