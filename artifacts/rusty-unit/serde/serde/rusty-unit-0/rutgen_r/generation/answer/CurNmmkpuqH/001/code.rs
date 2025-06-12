// Answer 0

#[test]
fn test_serialize_f32_err() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), String> {
            Err("unsupported type".to_string())
        }
    }

    let serializer = TestSerializer;

    let result = serializer.serialize_f32(3.14);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "unsupported type");
}

#[test]
fn test_serialize_f32_err_zero() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), String> {
            Err("unsupported type".to_string())
        }
    }

    let serializer = TestSerializer;

    let result = serializer.serialize_f32(0.0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "unsupported type");
}

#[test]
fn test_serialize_f32_err_negative() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), String> {
            Err("unsupported type".to_string())
        }
    }

    let serializer = TestSerializer;

    let result = serializer.serialize_f32(-1.23);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "unsupported type");
}

