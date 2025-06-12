// Answer 0

#[derive(Debug)]
struct GetDisjointMutError {
    kind: String,
}

struct ExampleMap<K, V> {
    entries: Vec<(K, V)>,
}

impl<K, V> ExampleMap<K, V> {
    fn len(&self) -> usize {
        self.entries.len()
    }

    fn get_disjoint_opt_mut<const N: usize>(
        &mut self,
        indices: [Option<usize>; N],
    ) -> Result<[Option<(&K, &mut V)>; N], GetDisjointMutError> {
        // Implementation as provided...
        let len = self.len();
        for i in 0..N {
            if let Some(idx) = indices[i] {
                if idx >= len {
                    return Err(GetDisjointMutError { kind: "IndexOutOfBounds".to_string() });
                } else if indices[..i].contains(&Some(idx)) {
                    return Err(GetDisjointMutError { kind: "OverlappingIndices".to_string() });
                }
            }
        }

        let entries_ptr = self.entries.as_mut_ptr();
        let out = indices.map(|idx_opt| {
            match idx_opt {
                Some(idx) => {
                    let kv = unsafe { (*(entries_ptr.add(idx))).1 };
                    Some((&self.entries[idx].0, kv))
                }
                None => None,
            }
        });

        Ok(out)
    }
}

#[test]
fn test_get_disjoint_opt_mut_valid_case() {
    let mut example_map = ExampleMap {
        entries: vec![
            (1, "a"),
            (2, "b"),
            (3, "c"),
        ],
    };
    let result: Result<[Option<(&i32, &mut &str)>; 2], GetDisjointMutError> = example_map.get_disjoint_opt_mut([Some(0), Some(2)]);
    assert!(result.is_ok());
}

#[test]
fn test_get_disjoint_opt_mut_out_of_bounds() {
    let mut example_map = ExampleMap {
        entries: vec![
            (1, "a"),
            (2, "b"),
            (3, "c"),
        ],
    };
    let result: Result<[Option<(&i32, &mut &str)>; 1], GetDisjointMutError> = example_map.get_disjoint_opt_mut([Some(3)]);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.kind, "IndexOutOfBounds");
    }
}

#[test]
fn test_get_disjoint_opt_mut_overlapping_indices() {
    let mut example_map = ExampleMap {
        entries: vec![
            (1, "a"),
            (2, "b"),
            (3, "c"),
        ],
    };
    let result: Result<[Option<(&i32, &mut &str)>; 2], GetDisjointMutError> = example_map.get_disjoint_opt_mut([Some(0), Some(0)]);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.kind, "OverlappingIndices");
    }
}

#[test]
fn test_get_disjoint_opt_mut_mixed_case() {
    let mut example_map = ExampleMap {
        entries: vec![
            (1, "a"),
            (2, "b"),
            (3, "c"),
        ],
    };
    let result: Result<[Option<(&i32, &mut &str)>; 3], GetDisjointMutError> = example_map.get_disjoint_opt_mut([Some(0), None, Some(2)]);
    assert!(result.is_ok());
}

#[test]
fn test_get_disjoint_opt_mut_empty_map() {
    let mut example_map = ExampleMap::<i32, &str> {
        entries: vec![],
    };
    let result: Result<[Option<(&i32, &mut &str)>; 1], GetDisjointMutError> = example_map.get_disjoint_opt_mut([Some(0)]);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.kind, "IndexOutOfBounds");
    }
}

