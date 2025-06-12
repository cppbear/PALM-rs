// Answer 0

#[test]
fn test_split_first_non_empty_slice() {
    struct TestIndexMap {
        entries: Vec<(&'static str, i32)>,
    }

    impl TestIndexMap {
        fn from_slice(slice: &[(&'static str, i32)]) -> Self {
            Self {
                entries: slice.to_vec(),
            }
        }

        fn refs(&self) -> (&'static str, &i32) {
            (&self.entries[0].0, &self.entries[0].1)
        }

        pub fn split_first(&self) -> Option<((&str, &i32), &Self)> {
            if let [first, rest @ ..] = &self.entries[..] {
                Some((self.refs(), Self::from_slice(rest)))
            } else {
                None
            }
        }
    }

    let map = TestIndexMap {
        entries: vec![("key1", 1), ("key2", 2)],
    };

    let result = map.split_first();
    assert!(result.is_some());

    if let Some(((first_key, first_value), rest)) = result {
        assert_eq!(first_key, "key1");
        assert_eq!(*first_value, 1);
        assert_eq!(rest.entries.len(), 1);
        assert_eq!(rest.entries[0], ("key2", 2));
    }
}

#[test]
fn test_split_first_empty_slice() {
    struct TestIndexMap {
        entries: Vec<(&'static str, i32)>,
    }

    impl TestIndexMap {
        fn from_slice(slice: &[(&'static str, i32)]) -> Self {
            Self {
                entries: slice.to_vec(),
            }
        }

        pub fn split_first(&self) -> Option<((&str, &i32), &Self)> {
            if let [first, rest @ ..] = &self.entries[..] {
                Some((first, Self::from_slice(rest)))
            } else {
                None
            }
        }
    }

    let empty_map = TestIndexMap { entries: vec![] };

    let result = empty_map.split_first();
    assert!(result.is_none());
}

