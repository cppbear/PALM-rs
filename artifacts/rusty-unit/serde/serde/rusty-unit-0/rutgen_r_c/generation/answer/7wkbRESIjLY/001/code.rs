// Answer 0

#[test]
fn test_end_function() {
    struct MockSerializeMap;

    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = std::convert::Infallible; // Using Infallible as there are no error cases.

        fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut mock_map = MockSerializeMap;
    let serializer = FlatMapSerializeMap(&mut mock_map);
    let result = serializer.end();
    assert_eq!(result, Ok(()));
}

