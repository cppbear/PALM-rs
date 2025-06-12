// Answer 0

#[test]
fn test_split_last_non_empty() {
    struct MockIndexMap {
        entries: Vec<(String, i32)>,
    }
    
    impl MockIndexMap {
        fn from_slice(slice: &[(String, i32)]) -> Self {
            MockIndexMap {
                entries: slice.to_vec(),
            }
        }

        fn split_last(&self) -> Option<((&String, &i32), &Self)> {
            if let [rest @ .., last] = &self.entries[..] {
                Some((last, Self::from_slice(rest)))
            } else {
                None
            }
        }
    }

    let mut map = MockIndexMap {
        entries: vec![
            ("key1".to_string(), 1),
            ("key2".to_string(), 2),
            ("key3".to_string(), 3),
        ],
    };

    let result = map.split_last();
    assert!(result.is_some());

    if let Some(((last_key, last_value), rest_map)) = result {
        assert_eq!(last_key, &"key3".to_string());
        assert_eq!(last_value, &3);
        assert_eq!(rest_map.entries.len(), 2);
        assert_eq!(rest_map.entries[0], ("key1".to_string(), 1));
        assert_eq!(rest_map.entries[1], ("key2".to_string(), 2));
    }
}

#[test]
fn test_split_last_empty() {
    struct MockIndexMap {
        entries: Vec<(String, i32)>,
    }

    impl MockIndexMap {
        fn from_slice(slice: &[(String, i32)]) -> Self {
            MockIndexMap {
                entries: slice.to_vec(),
            }
        }

        fn split_last(&self) -> Option<((&String, &i32), &Self)> {
            if let [rest @ .., last] = &self.entries[..] {
                Some((last, Self::from_slice(rest)))
            } else {
                None
            }
        }
    }

    let empty_map = MockIndexMap { entries: vec![] };
    let result = empty_map.split_last();
    assert!(result.is_none());
}

