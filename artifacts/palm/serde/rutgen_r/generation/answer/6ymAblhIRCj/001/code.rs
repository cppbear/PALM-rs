// Answer 0

#[test]
fn test_serialize_u8_invalid_type() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(_: Unsupported) -> Result<(), String> {
            Err("Unsupported Integer".to_string())
        }
        
        fn serialize_u8(self, _: u8) -> Result<(), String> {
            Err(Self::bad_type(Unsupported::Integer))
        }
    }

    struct Unsupported;
    impl Unsupported {
        const Integer: Self = Unsupported;
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_u8(250); // Using a valid u8 input
    assert_eq!(result, Err("Unsupported Integer".to_string()));
}

#[test]
fn test_serialize_u8_zero() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(_: Unsupported) -> Result<(), String> {
            Err("Unsupported Integer".to_string())
        }
        
        fn serialize_u8(self, _: u8) -> Result<(), String> {
            Err(Self::bad_type(Unsupported::Integer))
        }
    }

    struct Unsupported;
    impl Unsupported {
        const Integer: Self = Unsupported;
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_u8(0); // Edge case with u8 value of 0
    assert_eq!(result, Err("Unsupported Integer".to_string()));
}

#[test]
fn test_serialize_u8_max_value() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(_: Unsupported) -> Result<(), String> {
            Err("Unsupported Integer".to_string())
        }
        
        fn serialize_u8(self, _: u8) -> Result<(), String> {
            Err(Self::bad_type(Unsupported::Integer))
        }
    }

    struct Unsupported;
    impl Unsupported {
        const Integer: Self = Unsupported;
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_u8(u8::MAX); // Edge case with u8 value of 255
    assert_eq!(result, Err("Unsupported Integer".to_string()));
}

