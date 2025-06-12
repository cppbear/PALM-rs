// Answer 0

#[test]
fn test_serialize_some() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), String> {
            Err("Unsupported type".into())
        }
        
        fn serialize_some<T>(&self, _: &T) -> Result<(), String>
        where
            T: ?Sized + Serialize,
        {
            Err(self.bad_type(Unsupported::Optional))
        }
    }

    struct Unsupported;

    let serializer = TestSerializer;
    let result = serializer.serialize_some(&None::<i32>);
    assert_eq!(result, Err("Unsupported type".into()));
}

