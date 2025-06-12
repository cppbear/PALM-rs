// Answer 0

#[test]
fn test_size_hint_none() {
    struct TestMap {
        data: Vec<(String, Value)>,
    }

    impl<'de> IntoIterator for TestMap {
        type Item = (String, Value);
        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self.data.into_iter()
        }
    }

    let map = TestMap {
        data: vec![("key1".to_string(), Value::Null)],
    };

    let iter = map.into_iter();
    let deserializer = MapRefDeserializer {
        iter,
        value: None,
    };

    assert_eq!(deserializer.size_hint(), None);
}

#[test]
fn test_size_hint_some() {
    struct TestMap {
        data: Vec<(String, Value)>,
    }

    impl<'de> IntoIterator for TestMap {
        type Item = (String, Value);
        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self.data.into_iter()
        }
    }

    let map = TestMap {
        data: vec![
            ("key1".to_string(), Value::Null),
            ("key2".to_string(), Value::Bool(true)),
        ],
    };

    let iter = map.into_iter();
    let deserializer = MapRefDeserializer {
        iter,
        value: None,
    };
    
    assert_eq!(deserializer.size_hint(), None);
}

#[test]
fn test_size_hint_equal_lower_upper() {
    struct TestMap {
        data: Vec<(String, Value)>,
    }

    impl<'de> IntoIterator for TestMap {
        type Item = (String, Value);
        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self.data.into_iter()
        }
    }

    let map = TestMap {
        data: vec![
            ("key1".to_string(), Value::Null),
            ("key2".to_string(), Value::Bool(false)),
            ("key3".to_string(), Value::Number(Number::from(10))),
        ],
    };

    let iter = map.into_iter();
    let deserializer = MapRefDeserializer {
        iter,
        value: None,
    };

    assert_eq!(deserializer.size_hint(), None);
}

