// Answer 0

#[test]
fn test_get_disjoint_opt_mut_valid_case() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn new(entries: Vec<(K, V)>) -> Self {
            TestMap { entries }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }
    }

    #[derive(Debug, PartialEq)]
    enum GetDisjointMutError {
        IndexOutOfBounds,
        OverlappingIndices,
    }

    let mut map = TestMap::new(vec![(1, "a"), (2, "b"), (3, "c")]);
    let indices = [Some(0), Some(1), None]; // Indices are valid and non-overlapping

    let result: Result<[Option<(&i32, &mut &str)>; 3], GetDisjointMutError> =
        map.get_disjoint_opt_mut(indices);

    assert!(result.is_ok());
}

#[test]
fn test_get_disjoint_opt_mut_index_out_of_bounds() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn new(entries: Vec<(K, V)>) -> Self {
            TestMap { entries }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }
    }

    #[derive(Debug, PartialEq)]
    enum GetDisjointMutError {
        IndexOutOfBounds,
        OverlappingIndices,
    }

    let mut map = TestMap::new(vec![(1, "a"), (2, "b"), (3, "c")]);
    let indices = [Some(0), Some(3), None]; // Index 3 is out of bounds

    let result: Result<[Option<(&i32, &mut &str)>; 3], GetDisjointMutError> =
        map.get_disjoint_opt_mut(indices);

    assert_eq!(result, Err(GetDisjointMutError::IndexOutOfBounds));
}

#[test]
fn test_get_disjoint_opt_mut_overlapping_indices() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn new(entries: Vec<(K, V)>) -> Self {
            TestMap { entries }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }
    }

    #[derive(Debug, PartialEq)]
    enum GetDisjointMutError {
        IndexOutOfBounds,
        OverlappingIndices,
    }

    let mut map = TestMap::new(vec![(1, "a"), (2, "b"), (3, "c")]);
    let indices = [Some(0), Some(0), None]; // Indices are overlapping

    let result: Result<[Option<(&i32, &mut &str)>; 3], GetDisjointMutError> =
        map.get_disjoint_opt_mut(indices);

    assert_eq!(result, Err(GetDisjointMutError::OverlappingIndices));
}

