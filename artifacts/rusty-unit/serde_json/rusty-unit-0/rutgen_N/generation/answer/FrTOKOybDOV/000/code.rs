// Answer 0

#[test]
fn test_serialize_tuple_err() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_tuple(self, _len: usize) -> Result<(), String> {
            Err(String::from("key must be a string"))
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_tuple(2);
    assert_eq!(result, Err(String::from("key must be a string")));
}

