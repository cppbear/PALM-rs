// Answer 0

#[test]
fn test_serialize_f64_returns_err_for_float() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), String> {
            Err("Unsupported type: Float".to_string())
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_f64(3.14);
    assert_eq!(result, Err("Unsupported type: Float".to_string()));
}

#[test]
fn test_serialize_f64_returns_err_for_negative_float() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), String> {
            Err("Unsupported type: Float".to_string())
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_f64(-2.71);
    assert_eq!(result, Err("Unsupported type: Float".to_string()));
}

#[test]
fn test_serialize_f64_returns_err_for_zero_float() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), String> {
            Err("Unsupported type: Float".to_string())
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_f64(0.0);
    assert_eq!(result, Err("Unsupported type: Float".to_string()));
}

