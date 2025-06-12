// Answer 0

#[test]
fn test_split_first_mut_empty() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn from_mut_slice(slice: &mut [(K, V)]) -> Self {
            Self {
                entries: slice.to_vec(),
            }
        }

        fn split_first_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
            if let [first, rest @ ..] = &mut self.entries {
                Some((first.ref_mut(), Self::from_mut_slice(rest)))
            } else {
                None
            }
        }
    }

    let mut empty_map: TestMap<i32, String> = TestMap { entries: Vec::new() };
    let result = empty_map.split_first_mut();
    assert_eq!(result, None);
}

#[test]
fn test_split_first_mut_single_entry() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn from_mut_slice(slice: &mut [(K, V)]) -> Self {
            Self {
                entries: slice.to_vec(),
            }
        }

        fn split_first_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
            if let [first, rest @ ..] = &mut self.entries {
                Some((first.ref_mut(), Self::from_mut_slice(rest)))
            } else {
                None
            }
        }
    }

    let mut single_entry_map: TestMap<i32, String> = TestMap { entries: vec![(1, String::from("a"))] };
    let result = single_entry_map.split_first_mut();
    assert_ne!(result, None);
}

#[test]
fn test_split_first_mut_multiple_entries() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn from_mut_slice(slice: &mut [(K, V)]) -> Self {
            Self {
                entries: slice.to_vec(),
            }
        }

        fn split_first_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
            if let [first, rest @ ..] = &mut self.entries {
                Some((first.ref_mut(), Self::from_mut_slice(rest)))
            } else {
                None
            }
        }
    }

    let mut multiple_entry_map: TestMap<i32, String> = TestMap { entries: vec![(1, String::from("a")), (2, String::from("b"))] };
    let result = multiple_entry_map.split_first_mut();
    assert_ne!(result, None);
}

