// Answer 0

#[test]
fn test_end() {
    use crate::ser::{SerializeMap, Serializer};

    struct TestSerializeMap;

    impl SerializeMap for TestSerializeMap {
        type Error = ();

        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<(), Self::Error>
        where
            K: ?Sized + Serialize,
            V: ?Sized + Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut map = TestSerializeMap;

    let mut serializer = FlatMapSerializeStruct(&mut map);
    let result = serializer.end();

    assert_eq!(result, Ok(()));
}

