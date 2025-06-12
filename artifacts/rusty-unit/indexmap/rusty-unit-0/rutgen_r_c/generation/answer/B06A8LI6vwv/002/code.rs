// Answer 0

#[test]
fn test_get_disjoint_mut_valid_indices() {
    struct TestMap {
        entries: Vec<Bucket<i32, &str>>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: vec![
                    Bucket { hash: 0, key: 1, value: "one" },
                    Bucket { hash: 0, key: 2, value: "two" },
                    Bucket { hash: 0, key: 3, value: "three" },
                ],
            }
        }
        
        fn get_disjoint_mut<const N: usize>(
            &mut self,
            indices: [usize; N],
        ) -> Result<[(&i32, &mut &str); N], GetDisjointMutError> {
            let key_values = self.get_disjoint_opt_mut(indices.map(Some))?;
            Ok(key_values.map(Option::unwrap))
        }

        fn get_disjoint_opt_mut<const N: usize>(
            &mut self,
            indices: [Option<usize>; N],
        ) -> Result<[Option<(&i32, &mut &str)>; N], GetDisjointMutError> {
            let mut result = [None; N];
            let mut unique_indices = std::collections::HashSet::new();
            for (i, &index) in indices.iter().enumerate() {
                match index {
                    Some(i) if i < self.entries.len() && unique_indices.insert(i) => {
                        result[i] = Some((&self.entries[i].key, &mut self.entries[i].value));
                    },
                    _ => return Err(GetDisjointMutError::IndexOutOfBounds),
                }
            }
            Ok(result)
        }
    }

    let mut map = TestMap::new();
    let result = map.get_disjoint_mut([0, 1]).unwrap();

    assert_eq!(result[0].0, &1);
    assert_eq!(result[0].1, &mut "one");
    assert_eq!(result[1].0, &2);
    assert_eq!(result[1].1, &mut "two");
}

#[test]
#[should_panic(expected = "IndexOutOfBounds")]
fn test_get_disjoint_mut_out_of_bounds() {
    struct TestMap {
        entries: Vec<Bucket<i32, &str>>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: vec![
                    Bucket { hash: 0, key: 1, value: "one" },
                    Bucket { hash: 0, key: 2, value: "two" },
                ],
            }
        }

        fn get_disjoint_mut<const N: usize>(
            &mut self,
            indices: [usize; N],
        ) -> Result<[(&i32, &mut &str); N], GetDisjointMutError> {
            let key_values = self.get_disjoint_opt_mut(indices.map(Some))?;
            Ok(key_values.map(Option::unwrap))
        }

        fn get_disjoint_opt_mut<const N: usize>(
            &mut self,
            indices: [Option<usize>; N],
        ) -> Result<[Option<(&i32, &mut &str)>; N], GetDisjointMutError> {
            let mut result = [None; N];
            let mut unique_indices = std::collections::HashSet::new();
            for (i, &index) in indices.iter().enumerate() {
                match index {
                    Some(i) if i < self.entries.len() && unique_indices.insert(i) => {
                        result[i] = Some((&self.entries[i].key, &mut self.entries[i].value));
                    },
                    _ => return Err(GetDisjointMutError::IndexOutOfBounds),
                }
            }
            Ok(result)
        }
    }

    let mut map = TestMap::new();
    let _ = map.get_disjoint_mut([0, 2]).unwrap();
}

#[test]
fn test_get_disjoint_mut_overlapping_indices() {
    struct TestMap {
        entries: Vec<Bucket<i32, &str>>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: vec![
                    Bucket { hash: 0, key: 1, value: "one" },
                    Bucket { hash: 0, key: 2, value: "two" },
                ],
            }
        }

        fn get_disjoint_mut<const N: usize>(
            &mut self,
            indices: [usize; N],
        ) -> Result<[(&i32, &mut &str); N], GetDisjointMutError> {
            let key_values = self.get_disjoint_opt_mut(indices.map(Some))?;
            Ok(key_values.map(Option::unwrap))
        }

        fn get_disjoint_opt_mut<const N: usize>(
            &mut self,
            indices: [Option<usize>; N],
        ) -> Result<[Option<(&i32, &mut &str)>; N], GetDisjointMutError> {
            let mut result = [None; N];
            let mut unique_indices = std::collections::HashSet::new();
            for (i, &index) in indices.iter().enumerate() {
                match index {
                    Some(i) if i < self.entries.len() && unique_indices.insert(i) => {
                        result[i] = Some((&self.entries[i].key, &mut self.entries[i].value));
                    },
                    _ => return Err(GetDisjointMutError::OverlappingIndices),
                }
            }
            Ok(result)
        }
    }

    let mut map = TestMap::new();
    let result = map.get_disjoint_mut([0, 0]).map_err(|e| match e {
        GetDisjointMutError::OverlappingIndices => {},
        _ => panic!("Unexpected error"),
    });
    assert!(result.is_err());
}

