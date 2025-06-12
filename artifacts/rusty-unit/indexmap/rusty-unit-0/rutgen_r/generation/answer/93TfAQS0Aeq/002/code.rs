// Answer 0

#[test]
fn test_split_last_mut_empty() {
    struct Map<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> Map<K, V> {
        fn from_mut_slice(slice: &mut [(K, V)]) -> Self {
            Self {
                entries: slice.to_vec(),
            }
        }

        pub fn split_last_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
            if let [rest @ .., last] = &mut self.entries {
                Some((last, Self::from_mut_slice(rest)))
            } else {
                None
            }
        }
    }

    let mut map: Map<i32, i32> = Map { entries: Vec::new() };
    let result = map.split_last_mut();
    assert!(result.is_none());
}

#[test]
fn test_split_last_mut_single() {
    struct Map<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> Map<K, V> {
        fn from_mut_slice(slice: &mut [(K, V)]) -> Self {
            Self {
                entries: slice.to_vec(),
            }
        }

        pub fn split_last_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
            if let [rest @ .., last] = &mut self.entries {
                Some((last, Self::from_mut_slice(rest)))
            } else {
                None
            }
        }
    }

    let mut map: Map<i32, i32> = Map {
        entries: vec![(1, 10)],
    };
    let result = map.split_last_mut();
    assert!(result.is_none());
}

