// Answer 0

#[test]
fn test_split_first_non_empty() {
    struct TestEntry<K, V> {
        key: K,
        value: V,
    }

    struct TestSlice {
        entries: Vec<TestEntry<i32, &str>>,
    }

    impl TestSlice {
        fn from_slice(slice: &[TestEntry<i32, &str>]) -> Self {
            Self {
                entries: slice.to_vec(),
            }
        }

        pub fn split_first(&self) -> Option<(&i32, Self)> {
            if let [first, rest @ ..] = &self.entries[..] {
                Some((&first.key, Self::from_slice(rest)))
            } else {
                None
            }
        }
    }

    let entry1 = TestEntry { key: 1, value: "one" };
    let entry2 = TestEntry { key: 2, value: "two" };
    let test_slice = TestSlice {
        entries: vec![entry1, entry2],
    };

    let result = test_slice.split_first();
    assert!(result.is_some());
    if let Some((key, rest)) = result {
        assert_eq!(*key, 1);
        assert_eq!(rest.entries.len(), 1);
        assert_eq!(rest.entries[0].key, 2);
    }
}

#[test]
fn test_split_first_empty() {
    struct TestEntry<K, V> {
        key: K,
        value: V,
    }

    struct TestSlice {
        entries: Vec<TestEntry<i32, &str>>,
    }

    impl TestSlice {
        fn from_slice(slice: &[TestEntry<i32, &str>]) -> Self {
            Self {
                entries: slice.to_vec(),
            }
        }

        pub fn split_first(&self) -> Option<(&i32, Self)> {
            if let [first, rest @ ..] = &self.entries[..] {
                Some((&first.key, Self::from_slice(rest)))
            } else {
                None
            }
        }
    }

    let test_slice = TestSlice { entries: vec![] };

    let result = test_slice.split_first();
    assert!(result.is_none());
}

