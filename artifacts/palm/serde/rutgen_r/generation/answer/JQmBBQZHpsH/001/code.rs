// Answer 0

#[test]
fn test_serialize_i32_error() {
    struct Serializer;

    impl Serializer {
        fn bad_type(_: Unsupported) -> String {
            "Unsupported Integer".to_string()
        }
        
        fn serialize_i32(self, _: i32) -> Result<(), String> {
            Err(Self::bad_type(Unsupported::Integer))
        }
    }

    struct Unsupported;
    impl Unsupported {
        const Integer: Self = Self;
    }

    let serializer = Serializer;
    let result = serializer.serialize_i32(42); // invoking with an i32 value

    assert_eq!(result, Err("Unsupported Integer".to_string()));
}

#[test]
fn test_serialize_i32_zero() {
    struct Serializer;

    impl Serializer {
        fn bad_type(_: Unsupported) -> String {
            "Unsupported Integer".to_string()
        }
        
        fn serialize_i32(self, _: i32) -> Result<(), String> {
            Err(Self::bad_type(Unsupported::Integer))
        }
    }

    struct Unsupported;
    impl Unsupported {
        const Integer: Self = Self;
    }

    let serializer = Serializer;
    let result = serializer.serialize_i32(0); // invoking with zero value

    assert_eq!(result, Err("Unsupported Integer".to_string()));
}

#[test]
fn test_serialize_i32_negative() {
    struct Serializer;

    impl Serializer {
        fn bad_type(_: Unsupported) -> String {
            "Unsupported Integer".to_string()
        }
        
        fn serialize_i32(self, _: i32) -> Result<(), String> {
            Err(Self::bad_type(Unsupported::Integer))
        }
    }

    struct Unsupported;
    impl Unsupported {
        const Integer: Self = Self;
    }

    let serializer = Serializer;
    let result = serializer.serialize_i32(-10); // invoking with a negative i32 value

    assert_eq!(result, Err("Unsupported Integer".to_string()));
}

