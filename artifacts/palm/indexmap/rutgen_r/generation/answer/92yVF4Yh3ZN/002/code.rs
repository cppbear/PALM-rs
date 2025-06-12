// Answer 0

#[test]
fn test_split_last_empty() {
    struct TestSlice {
        entries: Vec<(i32, i32)>,
    }

    impl TestSlice {
        fn from_slice(slice: &[Self]) -> Self {
            Self {
                entries: slice.iter().map(|s| (s.entries[0].0, s.entries[0].1)).collect(),
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

    let test_slice = TestSlice { entries: Vec::new() };
    let result = test_slice.split_last();
    assert_eq!(result, None);
}

#[test]
fn test_split_last_single_element() {
    struct TestSlice {
        entries: Vec<(i32, i32)>,
    }

    impl TestSlice {
        fn from_slice(slice: &[Self]) -> Self {
            Self {
                entries: slice.iter().map(|s| (s.entries[0].0, s.entries[0].1)).collect(),
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

    let test_slice = TestSlice { entries: vec![(1, 1)] };
    let result = test_slice.split_last();
    assert_eq!(result, None);
}

