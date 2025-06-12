// Answer 0

#[test]
fn test_size_hint_some() {
    struct TestMap<K, V> {
        iter: Vec<(K, V)>,
    }

    impl<K, V> IntoIterator for TestMap<K, V> {
        type Item = (K, V);
        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter.into_iter()
        }
    }

    struct TestMapRefDeserializer<'de> {
        iter: <TestMap<String, i32> as IntoIterator>::IntoIter,
    }

    impl<'de> TestMapRefDeserializer<'de> {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_data = TestMap {
        iter: vec![("key1".to_string(), 1), ("key2".to_string(), 2)],
    };

    let deserializer = TestMapRefDeserializer {
        iter: test_data.into_iter(),
    };

    assert_eq!(deserializer.size_hint(), Some(2));
}

#[test]
fn test_size_hint_none() {
    struct TestMap<K, V> {
        iter: Vec<(K, V)>,
    }

    impl<K, V> IntoIterator for TestMap<K, V> {
        type Item = (K, V);
        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter.into_iter()
        }
    }

    struct TestMapRefDeserializer<'de> {
        iter: <TestMap<String, i32> as IntoIterator>::IntoIter,
    }

    impl<'de> TestMapRefDeserializer<'de> {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_data = TestMap {
        iter: vec![("key1".to_string(), 1)], // In this case, lower != upper
    };

    let deserializer = TestMapRefDeserializer {
        iter: test_data.into_iter(),
    };

    assert_eq!(deserializer.size_hint(), None);
}

