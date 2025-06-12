// Answer 0

#[test]
fn test_serialize_struct_error() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<(), &'static str> {
            Err("key must be a string")
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_struct("test", 1);
    assert_eq!(result, Err("key must be a string"));
}

