// Answer 0

#[derive(Debug)]
struct GetDisjointMutError {
    kind: String,
}

impl GetDisjointMutError {
    fn index_out_of_bounds() -> Self {
        GetDisjointMutError { kind: "IndexOutOfBounds".to_string() }
    }

    fn overlapping_indices() -> Self {
        GetDisjointMutError { kind: "OverlappingIndices".to_string() }
    }
}

struct MyMap<K, V> {
    entries: Vec<(K, V)>,
}

impl<K, V> MyMap<K, V> {
    fn len(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn get_disjoint_opt_mut<const N: usize>(
        &mut self,
        indices: [Option<usize>; N],
    ) -> Result<[Option<(&K, &mut V)>; N], GetDisjointMutError> {
        let len = self.len();
        for i in 0..N {
            if let Some(idx) = indices[i] {
                if idx >= len {
                    return Err(GetDisjointMutError::index_out_of_bounds());
                } else if indices[..i].contains(&Some(idx)) {
                    return Err(GetDisjointMutError::overlapping_indices());
                }
            }
        }

        let entries_ptr = self.entries.as_mut_ptr();
        let out = indices.map(|idx_opt| {
            match idx_opt {
                Some(idx) => {
                    let kv = unsafe { (*(entries_ptr.add(idx))).0 }; // Simplified for testing
                    Some((&kv.0, &mut kv.1)) // Assuming we can return a reference to V
                }
                None => None,
            }
        });

        Ok(out)
    }
}

#[test]
fn test_get_disjoint_opt_mut_index_out_of_bounds() {
    let mut my_map: MyMap<i32, i32> = MyMap { entries: vec![(1, 2), (3, 4)] };
    let indices = [Some(2), None]; // Valid length with an out-of-bounds index
    let result = my_map.get_disjoint_opt_mut::<2>(indices);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, "IndexOutOfBounds");
}

