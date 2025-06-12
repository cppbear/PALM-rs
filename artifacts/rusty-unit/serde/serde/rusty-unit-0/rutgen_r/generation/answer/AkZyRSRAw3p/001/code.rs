// Answer 0

#[test]
fn test_serialize_i8_should_return_err_for_unsupported_integer() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(_: Unsupported) -> &'static str {
            "Unsupported type"
        }
        
        fn serialize_i8(self, _: i8) -> Result<&'static str, &'static str> {
            Err(Self::bad_type(Unsupported::Integer))
        }
    }

    struct Unsupported;

    impl Unsupported {
        const Integer: () = ();
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i8(42);
    assert_eq!(result, Err("Unsupported type"));
}

