// Answer 0

#[test]
fn test_serialize_with_valid_entries() {
    use serde_json::Serializer;
    use serde::ser::SerializeMap;

    struct TestMap<'a> {
        data: Vec<(&'a str, i32)>,
    }

    impl<'a> TestMap<'a> {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    impl<'a> serde::ser::Serialize for TestMap<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            let mut map = serializer.serialize_map(Some(self.len()))?;
            for (k, v) in &self.data {
                map.serialize_entry(k, v)?;
            }
            map.end()
        }
    }

    let test_map = TestMap {
        data: vec![("key1", 1), ("key2", 2)],
    };

    let mut output = serde_json::Map::new();
    let result = test_map.serialize(&mut Serializer::new());

    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_with_empty_map() {
    use serde_json::Serializer;
    use serde::ser::SerializeMap;

    struct TestMap<'a> {
        data: Vec<(&'a str, i32)>,
    }

    impl<'a> TestMap<'a> {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    impl<'a> serde::ser::Serialize for TestMap<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            let mut map = serializer.serialize_map(Some(self.len()))?;
            for (k, v) in &self.data {
                map.serialize_entry(k, v)?;
            }
            map.end()
        }
    }

    let test_map = TestMap { data: vec![] };

    let _ = test_map.serialize(&mut Serializer::new());
}

