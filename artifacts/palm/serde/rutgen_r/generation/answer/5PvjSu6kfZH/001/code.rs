// Answer 0

#[test]
fn test_serialize_bytes_returns_err_for_byte_array() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(_: Unsupported) -> &'static str {
            "Unsupported ByteArray"
        }
        
        fn serialize_bytes(self, _: &[u8]) -> Result<&'static str, &'static str> {
            Err(Self::bad_type(Unsupported::ByteArray))
        }
    }

    struct Unsupported;

    #[allow(unused)]
    impl Unsupported {
        const ByteArray: Self = Unsupported;
    }
    
    let serializer = TestSerializer;

    let result = serializer.serialize_bytes(&[1, 2, 3, 4]);
    assert_eq!(result, Err("Unsupported ByteArray"));
}

