// Answer 0

#[test]
fn test_split_first_non_empty() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl TestMap<String, i32> {
        fn from_slice(entries: &[(String, i32)]) -> Self {
            Self {
                entries: entries.to_vec(),
            }
        }

        fn split_first(&self) -> Option<((&String, &i32), &Self)> {
            if let [first, rest @ ..] = &self.entries[..] {
                Some((first, &Self::from_slice(rest)))
            } else {
                None
            }
        }
    }

    let map = TestMap {
        entries: vec![
            ("key1".to_string(), 1),
            ("key2".to_string(), 2),
        ],
    };

    if let Some(((key, value), rest)) = map.split_first() {
        assert_eq!(key, &"key1".to_string());
        assert_eq!(*value, 1);
        assert_eq!(rest.entries.len(), 1);
        assert_eq!(rest.entries[0].0, "key2".to_string());
    } else {
        panic!("Expected a non-empty result");
    }
}

#[test]
fn test_split_first_empty() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl TestMap<String, i32> {
        fn from_slice(entries: &[(String, i32)]) -> Self {
            Self {
                entries: entries.to_vec(),
            }
        }

        fn split_first(&self) -> Option<((&String, &i32), &Self)> {
            if let [first, rest @ ..] = &self.entries[..] {
                Some((first, &Self::from_slice(rest)))
            } else {
                None
            }
        }
    }

    let map = TestMap { entries: vec![] };

    assert!(map.split_first().is_none());
}

