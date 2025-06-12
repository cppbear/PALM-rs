// Answer 0

#[test]
fn test_split_last_non_empty() {
    struct Entry {
        key: i32,
    }

    struct Slice {
        entries: Vec<Entry>,
    }

    impl Slice {
        fn from_slice(entries: &[Entry]) -> Self {
            Self {
                entries: entries.to_vec(),
            }
        }

        pub fn split_last(&self) -> Option<(&i32, Self)> {
            if let [rest @ .., last] = &self.entries[..] {
                Some((&last.key, Self::from_slice(rest)))
            } else {
                None
            }
        }
    }

    let slice = Slice {
        entries: vec![Entry { key: 1 }, Entry { key: 2 }, Entry { key: 3 }],
    };

    let result = slice.split_last();
    assert!(result.is_some());

    let (last_key, rest_slice) = result.unwrap();
    assert_eq!(*last_key, 3);
    assert_eq!(rest_slice.entries.len(), 2);
    assert_eq!(rest_slice.entries[0].key, 1);
    assert_eq!(rest_slice.entries[1].key, 2);
}

#[test]
fn test_split_last_empty() {
    struct Entry {
        key: i32,
    }

    struct Slice {
        entries: Vec<Entry>,
    }

    impl Slice {
        fn from_slice(entries: &[Entry]) -> Self {
            Self {
                entries: entries.to_vec(),
            }
        }

        pub fn split_last(&self) -> Option<(&i32, Self)> {
            if let [rest @ .., last] = &self.entries[..] {
                Some((&last.key, Self::from_slice(rest)))
            } else {
                None
            }
        }
    }

    let slice = Slice {
        entries: vec![],
    };

    let result = slice.split_last();
    assert!(result.is_none());
}

