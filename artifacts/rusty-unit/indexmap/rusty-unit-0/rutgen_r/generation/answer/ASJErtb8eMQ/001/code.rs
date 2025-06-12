// Answer 0

#[derive(Debug)]
struct GetDisjointMutError {
    reason: String,
}

struct Entry<K, V> {
    key: K,
    value: V,
}

struct SliceMap<K, V> {
    entries: Vec<Entry<K, V>>,
}

impl<K, V> SliceMap<K, V> {
    fn len(&self) -> usize {
        self.entries.len()
    }
    
    fn get_disjoint_opt_mut<const N: usize>(
        &mut self,
        indices: [Option<usize>; N],
    ) -> Result<[Option<(&K, &mut V)>; N], GetDisjointMutError> {
        let len = self.len();
        for i in 0..N {
            if let Some(idx) = indices[i] {
                if idx >= len {
                    return Err(GetDisjointMutError {
                        reason: "IndexOutOfBounds".to_string(),
                    });
                } else if indices[..i].contains(&Some(idx)) {
                    return Err(GetDisjointMutError {
                        reason: "OverlappingIndices".to_string(),
                    });
                }
            }
        }

        let entries_ptr = self.entries.as_mut_ptr();
        let out = indices.map(|idx_opt| {
            match idx_opt {
                Some(idx) => {
                    let kv = unsafe { (&mut (*(entries_ptr.add(idx))).value, &mut (*(entries_ptr.add(idx))).key) };
                    Some(kv)
                }
                None => None,
            }
        });

        Ok(out)
    }
}

#[test]
fn test_get_disjoint_opt_mut_valid_case() {
    let mut map = SliceMap {
        entries: vec![
            Entry { key: 'a', value: 1 },
            Entry { key: 'b', value: 2 },
            Entry { key: 'c', value: 3 },
        ],
    };
    
    let indices = [Some(0), Some(1), None];
    let result = map.get_disjoint_opt_mut(indices);
    
    assert!(result.is_ok());
    if let Ok(out) = result {
        assert_eq!(out[0], Some((&'a', &mut 1)));
        assert_eq!(out[1], Some((&'b', &mut 2)));
        assert_eq!(out[2], None);
    }
}

#[test]
fn test_get_disjoint_opt_mut_overlapping_indices() {
    let mut map = SliceMap {
        entries: vec![
            Entry { key: 'a', value: 1 },
            Entry { key: 'b', value: 2 },
            Entry { key: 'c', value: 3 },
        ],
    };
    
    let indices = [Some(0), Some(0), None];
    let result = map.get_disjoint_opt_mut(indices);
    
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.reason, "OverlappingIndices");
    }
}

#[test]
fn test_get_disjoint_opt_mut_index_out_of_bounds() {
    let mut map = SliceMap {
        entries: vec![
            Entry { key: 'a', value: 1 },
            Entry { key: 'b', value: 2 },
            Entry { key: 'c', value: 3 },
        ],
    };
    
    let indices = [Some(0), Some(2), Some(3)];  // 3 is out of bounds
    let result = map.get_disjoint_opt_mut(indices);
    
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.reason, "IndexOutOfBounds");
    }
}

#[test]
fn test_get_disjoint_opt_mut_all_none() {
    let mut map = SliceMap {
        entries: vec![
            Entry { key: 'a', value: 1 },
            Entry { key: 'b', value: 2 },
            Entry { key: 'c', value: 3 },
        ],
    };
    
    let indices = [None, None, None];
    let result = map.get_disjoint_opt_mut(indices);
    
    assert!(result.is_ok());
    if let Ok(out) = result {
        assert_eq!(out, [None, None, None]);
    }
}

#[test]
fn test_get_disjoint_opt_mut_one_none_one_some() {
    let mut map = SliceMap {
        entries: vec![
            Entry { key: 'a', value: 1 },
            Entry { key: 'b', value: 2 },
        ],
    };
    
    let indices = [Some(0), None];
    let result = map.get_disjoint_opt_mut(indices);
    
    assert!(result.is_ok());
    if let Ok(out) = result {
        assert_eq!(out[0], Some((&'a', &mut 1)));
        assert_eq!(out[1], None);
    }
}

