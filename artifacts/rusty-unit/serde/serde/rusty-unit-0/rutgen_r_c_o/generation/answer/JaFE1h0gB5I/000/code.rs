// Answer 0

#[test]
fn test_serialize_u32_bad_type() {
    struct Serializer;
    
    impl Serializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), &'static str> {
            Err("bad type")
        }
    }

    enum Unsupported {
        Integer,
    }

    let serializer = Serializer;
    let result = serializer.serialize_u32(42);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "bad type");
}

