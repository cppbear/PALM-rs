// Answer 0

#[test]
fn test_serialize_char() {
    struct MockMap;

    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = String;

        fn serialize_key(&mut self, _: &str) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
        fn serialize_entry<K: Serialize, V: Serialize>(&mut self, _: &K, _: &V) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap;
    let serializer = FlatMapSerializer(&mut map);

    let result = serializer.serialize_char('a');
    assert_eq!(result, Err(String::from("can only flatten structs and maps (got Char)")));
}

#[test]
fn test_serialize_char_with_different_character() {
    struct MockMap;

    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = String;

        fn serialize_key(&mut self, _: &str) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
        fn serialize_entry<K: Serialize, V: Serialize>(&mut self, _: &K, _: &V) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap;
    let serializer = FlatMapSerializer(&mut map);

    let result = serializer.serialize_char('Z');
    assert_eq!(result, Err(String::from("can only flatten structs and maps (got Char)")));
}

#[test]
fn test_serialize_char_non_ascii() {
    struct MockMap;

    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = String;

        fn serialize_key(&mut self, _: &str) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
        fn serialize_entry<K: Serialize, V: Serialize>(&mut self, _: &K, _: &V) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap;
    let serializer = FlatMapSerializer(&mut map);

    let result = serializer.serialize_char('ñ');
    assert_eq!(result, Err(String::from("can only flatten structs and maps (got Char)")));
}

