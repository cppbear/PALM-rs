// Answer 0

#[test]
fn test_split_first_empty_slice() {
    struct TestIndexMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestIndexMap {
        fn from_slice(slice: &[(i32, i32)]) -> Self {
            TestIndexMap {
                entries: slice.to_vec(),
            }
        }

        fn split_first(&self) -> Option<((&i32, &i32), &Self)> {
            if let [first, rest @ ..] = &self.entries[..] {
                Some((first, Self::from_slice(rest)))
            } else {
                None
            }
        }
    }

    let empty_map = TestIndexMap { entries: Vec::new() };
    let result = empty_map.split_first();
    assert_eq!(result, None);
}

#[test]
fn test_split_first_single_element_slice() {
    struct TestIndexMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestIndexMap {
        fn from_slice(slice: &[(i32, i32)]) -> Self {
            TestIndexMap {
                entries: slice.to_vec(),
            }
        }

        fn split_first(&self) -> Option<((&i32, &i32), &Self)> {
            if let [first, rest @ ..] = &self.entries[..] {
                Some((first, Self::from_slice(rest)))
            } else {
                None
            }
        }
    }

    let single_element_map = TestIndexMap { entries: vec![(1, 2)] };
    let result = single_element_map.split_first();
    assert_eq!(result, None);
}

#[test]
fn test_split_first_two_elements_slice() {
    struct TestIndexMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestIndexMap {
        fn from_slice(slice: &[(i32, i32)]) -> Self {
            TestIndexMap {
                entries: slice.to_vec(),
            }
        }

        fn split_first(&self) -> Option<((&i32, &i32), &Self)> {
            if let [first, rest @ ..] = &self.entries[..] {
                Some((first, Self::from_slice(rest)))
            } else {
                None
            }
        }
    }

    let two_elements_map = TestIndexMap { entries: vec![(1, 2), (3, 4)] };
    let result = two_elements_map.split_first();
    assert_eq!(result, None);
}

