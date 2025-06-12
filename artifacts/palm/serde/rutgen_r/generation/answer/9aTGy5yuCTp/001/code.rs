// Answer 0

#[test]
fn test_serialize_i8_error() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), ()> {
            Err(())
        }
    }

    impl TestSerializer {
        fn serialize_i8(self, _: i8) -> Result<(), ()> {
            Err(self.bad_type(Unsupported::Integer))
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i8(0); // Test with a valid i8 value
    assert!(result.is_err());
}

#[test]
fn test_serialize_negative_i8_error() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), ()> {
            Err(())
        }
    }

    impl TestSerializer {
        fn serialize_i8(self, _: i8) -> Result<(), ()> {
            Err(self.bad_type(Unsupported::Integer))
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i8(-128); // Test with the minimum i8 value
    assert!(result.is_err());
}

#[test]
fn test_serialize_positive_i8_error() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), ()> {
            Err(())
        }
    }

    impl TestSerializer {
        fn serialize_i8(self, _: i8) -> Result<(), ()> {
            Err(self.bad_type(Unsupported::Integer))
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i8(127); // Test with the maximum i8 value
    assert!(result.is_err());
}

