// Answer 0

#[test]
fn test_split_last_non_empty() {
    struct TestSlice {
        entries: Vec<(i32, i32)>,
    }

    impl TestSlice {
        pub fn from_slice(slice: &[(i32, i32)]) -> Self {
            Self {
                entries: slice.to_vec(),
            }
        }

        pub fn split_last(&self) -> Option<(&i32, &Self)> {
            if let [rest @ .., last] = &self.entries[..] {
                Some((&last.0, Self::from_slice(rest)))
            } else {
                None
            }
        }
    }

    let slice = TestSlice {
        entries: vec![(1, 10), (2, 20), (3, 30)],
    };
    
    let result = slice.split_last();
    assert!(result.is_some());
    let (last_key, rest_slice) = result.unwrap();
    assert_eq!(*last_key, 3);
    assert_eq!(rest_slice.entries, vec![(1, 10), (2, 20)]);
}

#[test]
fn test_split_last_empty() {
    struct TestSlice {
        entries: Vec<(i32, i32)>,
    }

    impl TestSlice {
        pub fn from_slice(slice: &[(i32, i32)]) -> Self {
            Self {
                entries: slice.to_vec(),
            }
        }

        pub fn split_last(&self) -> Option<(&i32, &Self)> {
            if let [rest @ .., last] = &self.entries[..] {
                Some((&last.0, Self::from_slice(rest)))
            } else {
                None
            }
        }
    }

    let slice = TestSlice {
        entries: vec![],
    };

    let result = slice.split_last();
    assert!(result.is_none());
}

