// Answer 0

#[test]
fn test_new_with_empty_map() {
    struct TestValue;

    impl IntoIterator for Map<String, TestValue> {
        type Item = (String, TestValue);
        type IntoIter = std::vec::IntoIter<(String, TestValue)>;

        fn into_iter(self) -> Self::IntoIter {
            Vec::new().into_iter() // empty map
        }
    }

    let empty_map: Map<String, TestValue> = Map { map: () };
    let deserializer = MapDeserializer::new(empty_map);
    assert!(deserializer.iter.into_iter().len() == 0);
}

#[test]
fn test_new_with_non_empty_map() {
    struct TestValue;

    impl IntoIterator for Map<String, TestValue> {
        type Item = (String, TestValue);
        type IntoIter = std::vec::IntoIter<(String, TestValue)>;

        fn into_iter(self) -> Self::IntoIter {
            vec![("key".to_string(), TestValue)].into_iter()
        }
    }

    let non_empty_map: Map<String, TestValue> = Map { map: () };
    let deserializer = MapDeserializer::new(non_empty_map);
    assert!(deserializer.iter.into_iter().len() == 1);
}

