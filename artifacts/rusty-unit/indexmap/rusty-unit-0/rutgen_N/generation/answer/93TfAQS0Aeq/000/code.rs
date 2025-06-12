// Answer 0

#[test]
fn test_split_last_mut_with_entries() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn from_mut_slice(slice: &mut [(K, V)]) -> &mut Self {
            unsafe { &mut *(slice as *mut _ as *mut TestMap<K, V>) }
        }

        pub fn split_last_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
            if let [rest @ .., last] = &mut self.entries[..] {
                Some((last, Self::from_mut_slice(rest)))
            } else {
                None
            }
        }
    }

    let mut test_map = TestMap {
        entries: vec![(1, "a"), (2, "b"), (3, "c")],
    };

    let result = test_map.split_last_mut();
    assert!(result.is_some());

    if let Some(((key, value), rest)) = result {
        assert_eq!(*key, 3);
        *value = "z"; // Mutate the value
        assert_eq!(test_map.entries.last().unwrap().1, "z");
        assert_eq!(rest.entries.len(), 2);
        assert_eq!(rest.entries[0].0, 1);
    }
}

#[test]
fn test_split_last_mut_with_empty_entries() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn from_mut_slice(slice: &mut [(K, V)]) -> &mut Self {
            unsafe { &mut *(slice as *mut _ as *mut TestMap<K, V>) }
        }

        pub fn split_last_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
            if let [rest @ .., last] = &mut self.entries[..] {
                Some((last, Self::from_mut_slice(rest)))
            } else {
                None
            }
        }
    }

    let mut empty_test_map: TestMap<i32, &str> = TestMap { entries: vec![] };

    let result = empty_test_map.split_last_mut();
    assert!(result.is_none());
}

