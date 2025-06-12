// Answer 0

#[test]
fn test_get_disjoint_mut_valid_indices() {
    struct TestMap {
        data: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { data: vec![(0, 10), (1, 20), (2, 30), (3, 40)] }
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn get_disjoint_opt_mut<const N: usize>(&mut self, indices: [Option<usize>; N]) -> Result<[Option<(&usize, &mut usize)>; N], ()> {
            let mut result = [None; N];
            for (i, &index) in indices.iter().enumerate() {
                if let Some(idx) = index {
                    if idx < self.len() {
                        result[i] = Some((&self.data[idx].0, &mut self.data[idx].1));
                    } else {
                        return Err(());
                    }
                } else {
                    return Err(());
                }
            }
            Ok(result)
        }

        pub fn get_disjoint_mut<const N: usize>(
            &mut self,
            indices: [usize; N],
        ) -> Result<[(&usize, &mut usize); N], ()> {
            let indices = indices.map(Some);
            let key_values = self.get_disjoint_opt_mut(indices)?;
            Ok(key_values.map(Option::unwrap))
        }
    }

    let mut test_map = TestMap::new();
    let indices = [0, 2];
    let result = test_map.get_disjoint_mut(indices).unwrap();
    assert_eq!(result[0].0, &0);
    assert_eq!(result[1].0, &2);
    
    // To check if the mutable references work
    result[0].1 += 5;
    assert_eq!(test_map.data[0].1, 15);
}

#[test]
#[should_panic]
fn test_get_disjoint_mut_invalid_index() {
    struct TestMap {
        data: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { data: vec![(0, 10), (1, 20), (2, 30), (3, 40)] }
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn get_disjoint_opt_mut<const N: usize>(&mut self, indices: [Option<usize>; N]) -> Result<[Option<(&usize, &mut usize)>; N], ()> {
            let mut result = [None; N];
            for (i, &index) in indices.iter().enumerate() {
                if let Some(idx) = index {
                    if idx < self.len() {
                        result[i] = Some((&self.data[idx].0, &mut self.data[idx].1));
                    } else {
                        return Err(());
                    }
                } else {
                    return Err(());
                }
            }
            Ok(result)
        }

        pub fn get_disjoint_mut<const N: usize>(
            &mut self,
            indices: [usize; N],
        ) -> Result<[(&usize, &mut usize); N], ()> {
            let indices = indices.map(Some);
            let key_values = self.get_disjoint_opt_mut(indices)?;
            Ok(key_values.map(Option::unwrap))
        }
    }

    let mut test_map = TestMap::new();
    let indices = [0, 4]; // Index 4 is out of bounds
    let _ = test_map.get_disjoint_mut(indices).unwrap(); // This should panic
}

