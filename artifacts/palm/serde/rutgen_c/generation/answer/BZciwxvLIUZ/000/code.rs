// Answer 0

#[test]
fn test_serialize_newtype_variant_success() {
    struct TestSerializer;

    impl Serialize for TestSerializer {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Ok(())
        }
    }

    let mut formatter = TestSerializer;
    let result: fmt::Result = formatter.serialize_newtype_variant("Test", 0, "Variant", &formatter);
    assert_eq!(result, Err(fmt::Error));
}

#[test]
fn test_serialize_newtype_variant_failure() {
    struct FailingSerializer;

    impl Serialize for FailingSerializer {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Ok(())
        }
    }

    let mut formatter = FailingSerializer;
    let result: fmt::Result = formatter.serialize_newtype_variant("Test", 0, "Variant", &formatter);
    assert_eq!(result, Err(fmt::Error));
}

