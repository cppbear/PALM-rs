// Answer 0

#[test]
fn test_get_disjoint_mut_valid_indices() {
    struct TestMap {
        data: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { data: vec![(0, 1), (2, 3), (4, 5)] }
        }

        fn get_disjoint_mut<const N: usize>(
            &mut self,
            indices: [usize; N],
        ) -> Result<[(&usize, &mut usize); N], &'static str> {
            let mut key_values = Vec::new();
            for idx in indices.iter() {
                if *idx < self.data.len() {
                    let (key, value) = &mut self.data[*idx];
                    key_values.push((key, value));
                } else {
                    return Err("Index out of bounds");
                }
            }
            Ok(key_values.try_into().unwrap())
        }
    }

    let mut map = TestMap::new();
    let indices = [0, 1];
    let result: Result<[(&usize, &mut usize); 2], _> = map.get_disjoint_mut(indices);
    assert!(result.is_ok());
    let (key1, value1) = result.unwrap()[0];
    assert_eq!(*key1, 0);
    *value1 += 10;
    assert_eq!(*value1, 11);
}

#[test]
fn test_get_disjoint_mut_invalid_indices() {
    struct TestMap {
        data: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { data: vec![(0, 1), (2, 3), (4, 5)] }
        }

        fn get_disjoint_mut<const N: usize>(
            &mut self,
            indices: [usize; N],
        ) -> Result<[(&usize, &mut usize); N], &'static str> {
            let mut key_values = Vec::new();
            for idx in indices.iter() {
                if *idx < self.data.len() {
                    let (key, value) = &mut self.data[*idx];
                    key_values.push((key, value));
                } else {
                    return Err("Index out of bounds");
                }
            }
            Ok(key_values.try_into().unwrap())
        }
    }

    let mut map = TestMap::new();
    let indices = [0, 3]; // 3 is an invalid index
    let result: Result<[(&usize, &mut usize); 2], _> = map.get_disjoint_mut(indices);
    assert!(result.is_err());
    assert_eq!(result.err(), Some("Index out of bounds"));
}

#[test]
fn test_get_disjoint_mut_repeated_indices() {
    struct TestMap {
        data: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { data: vec![(0, 1), (2, 3), (4, 5)] }
        }

        fn get_disjoint_mut<const N: usize>(
            &mut self,
            indices: [usize; N],
        ) -> Result<[(&usize, &mut usize); N], &'static str> {
            let mut key_values = Vec::new();
            let mut seen = std::collections::HashSet::new();
            for idx in indices.iter() {
                if *idx < self.data.len() {
                    if !seen.insert(*idx) {
                        return Err("Duplicate indices");
                    }
                    let (key, value) = &mut self.data[*idx];
                    key_values.push((key, value));
                } else {
                    return Err("Index out of bounds");
                }
            }
            Ok(key_values.try_into().unwrap())
        }
    }

    let mut map = TestMap::new();
    let indices = [0, 0]; // Repeated index
    let result: Result<[(&usize, &mut usize); 2], _> = map.get_disjoint_mut(indices);
    assert!(result.is_err());
    assert_eq!(result.err(), Some("Duplicate indices"));
}

