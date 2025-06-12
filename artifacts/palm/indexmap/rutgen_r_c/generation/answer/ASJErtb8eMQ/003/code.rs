// Answer 0

#[test]
fn test_get_disjoint_opt_mut_overlapping_indices() {
    struct TestSlice {
        entries: [Bucket<i32, i32>; 3],
    }

    impl TestSlice {
        fn new() -> Self {
            Self {
                entries: [
                    Bucket { hash: 0, key: 1, value: 10 },
                    Bucket { hash: 0, key: 2, value: 20 },
                    Bucket { hash: 0, key: 3, value: 30 },
                ],
            }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn get_disjoint_opt_mut<const N: usize>(
            &mut self,
            indices: [Option<usize>; N],
        ) -> Result<[Option<(&i32, &mut i32)>; N], GetDisjointMutError> {
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

    let mut test_slice = TestSlice::new();
    
    // Testing overlapping indices
    let indices: [Option<usize>; 3] = [Some(0), Some(1), Some(0)];
    let result = test_slice.get_disjoint_opt_mut(indices);
    assert_eq!(result, Err(GetDisjointMutError::OverlappingIndices));
}

