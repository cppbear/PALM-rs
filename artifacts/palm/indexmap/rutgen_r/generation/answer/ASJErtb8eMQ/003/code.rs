// Answer 0

#[test]
fn test_get_disjoint_opt_mut_overlapping_indices() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: vec![(1, 10), (2, 20), (3, 30)],
            }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn get_disjoint_opt_mut<const N: usize>(
            &mut self,
            indices: [Option<usize>; N],
        ) -> Result<[Option<(&i32, &mut i32)>; N], GetDisjointMutError> {
            // Implementation of the function to be tested
            let len = self.len();
            for i in 0..N {
                if let Some(idx) = indices[i] {
                    if idx >= len {
                        return Err(GetDisjointMutError::IndexOutOfBounds);
                    } else if indices[..i].contains(&Some(idx)) {
                        return Err(GetDisjointMutError::OverlappingIndices);
                    }
                }
            }

            let entries_ptr = self.entries.as_mut_ptr();
            let out = indices.map(|idx_opt| {
                match idx_opt {
                    Some(idx) => {
                        let kv = unsafe { (*(entries_ptr.add(idx))).ref_mut() };
                        Some(kv)
                    }
                    None => None,
                }
            });

            Ok(out)
        }
    }

    #[derive(Debug)]
    enum GetDisjointMutError {
        IndexOutOfBounds,
        OverlappingIndices,
    }

    let mut test_map = TestMap::new();
    
    // Testing overlapping indices
    let indices = [Some(0), Some(0), None]; // Overlapping index at position 0
    let result = test_map.get_disjoint_opt_mut::<3>(indices);
    
    assert_eq!(result, Err(GetDisjointMutError::OverlappingIndices));
}

