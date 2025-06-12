// Answer 0

#[test]
fn test_index_from_hash_found() {
    struct TestEntry {
        key: u64,
    }

    struct TestMap {
        core: TestCore,
    }

    struct TestCore {
        entries: Vec<TestEntry>,
        indices: TestIndices,
    }

    struct TestIndices {
        indices: Vec<Option<usize>>,
    }

    impl TestIndices {
        fn find<F>(&self, hash: usize, mut eq: F) -> Option<&Option<usize>>
        where
            F: FnMut(&usize) -> bool,
        {
            self.indices.iter().position(|i| i.is_some() && eq(&i.unwrap())).map(|i| &self.indices[i])
        }
    }

    let entries = vec![
        TestEntry { key: 1 },
        TestEntry { key: 2 },
        TestEntry { key: 3 },
    ];

    let indices = TestIndices {
        indices: vec![Some(0), Some(1), None],
    };

    let test_map = TestMap {
        core: TestCore { entries, indices },
    };

    let result = test_map.index_from_hash(1, |key| *key == 2);
    assert_eq!(result, Some(1));
}

#[test]
fn test_index_from_hash_not_found() {
    struct TestEntry {
        key: u64,
    }

    struct TestMap {
        core: TestCore,
    }

    struct TestCore {
        entries: Vec<TestEntry>,
        indices: TestIndices,
    }

    struct TestIndices {
        indices: Vec<Option<usize>>,
    }

    impl TestIndices {
        fn find<F>(&self, hash: usize, mut eq: F) -> Option<&Option<usize>>
        where
            F: FnMut(&usize) -> bool,
        {
            self.indices.iter().position(|i| i.is_some() && eq(&i.unwrap())).map(|i| &self.indices[i])
        }
    }

    let entries = vec![
        TestEntry { key: 1 },
        TestEntry { key: 2 },
    ];

    let indices = TestIndices {
        indices: vec![Some(0), Some(1)],
    };

    let test_map = TestMap {
        core: TestCore { entries, indices },
    };

    let result = test_map.index_from_hash(1, |key| *key == 3);
    assert_eq!(result, None);
}

#[test]
fn test_index_from_hash_empty() {
    struct TestEntry {
        key: u64,
    }

    struct TestMap {
        core: TestCore,
    }

    struct TestCore {
        entries: Vec<TestEntry>,
        indices: TestIndices,
    }

    struct TestIndices {
        indices: Vec<Option<usize>>,
    }

    impl TestIndices {
        fn find<F>(&self, hash: usize, mut eq: F) -> Option<&Option<usize>>
        where
            F: FnMut(&usize) -> bool,
        {
            self.indices.iter().position(|i| i.is_some() && eq(&i.unwrap())).map(|i| &self.indices[i])
        }
    }

    let entries: Vec<TestEntry> = vec![];
    let indices = TestIndices { indices: vec![] };

    let test_map = TestMap {
        core: TestCore { entries, indices },
    };

    let result = test_map.index_from_hash(0, |key| *key == 0);
    assert_eq!(result, None);
}

