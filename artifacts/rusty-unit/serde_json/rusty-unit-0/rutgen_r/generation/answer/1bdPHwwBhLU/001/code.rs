// Answer 0

#[test]
fn test_serialize_str_valid() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_str(&self, value: &str) -> Result<()> {
            // Mock successful serialization
            if value.is_empty() {
                Err(Error::new(ErrorKind::InvalidInput, "Value cannot be empty"))
            } else {
                Ok(())
            }
        }
    }

    let serializer = TestSerializer;

    // Test with a valid string
    let result = serializer.serialize_str("Hello, world!");
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "Value cannot be empty")]
fn test_serialize_str_empty() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_str(&self, value: &str) -> Result<()> {
            // Mock successful serialization
            if value.is_empty() {
                panic!("Value cannot be empty");
            } else {
                Ok(())
            }
        }
    }

    let serializer = TestSerializer;

    // Test with an empty string which should panic
    let _ = serializer.serialize_str("");
}

