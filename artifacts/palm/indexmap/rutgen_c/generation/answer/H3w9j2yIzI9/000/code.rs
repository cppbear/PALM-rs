// Answer 0

#[test]
fn test_insert_sorted_new_value() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    struct SimpleIndexSet {
        items: Vec<i32>,
        hash_builder: TestHasher,
    }

    impl SimpleIndexSet {
        fn new() -> Self {
            Self {
                items: Vec::new(),
                hash_builder: TestHasher,
            }
        }

        fn insert_sorted(&mut self, value: i32) -> (usize, bool) {
            let pos = self.items.binary_search(&value);
            match pos {
                Ok(index) => (index, false),
                Err(index) => {
                    self.items.insert(index, value);
                    (index, true)
                }
            }
        }
    }

    let mut index_set = SimpleIndexSet::new();
    let (index, inserted) = index_set.insert_sorted(5);
    assert_eq!(index, 0);
    assert!(inserted);
}

#[test]
fn test_insert_sorted_existing_value() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    struct SimpleIndexSet {
        items: Vec<i32>,
        hash_builder: TestHasher,
    }

    impl SimpleIndexSet {
        fn new() -> Self {
            Self {
                items: Vec::new(),
                hash_builder: TestHasher,
            }
        }

        fn insert_sorted(&mut self, value: i32) -> (usize, bool) {
            let pos = self.items.binary_search(&value);
            match pos {
                Ok(index) => (index, false),
                Err(index) => {
                    self.items.insert(index, value);
                    (index, true)
                }
            }
        }
    }

    let mut index_set = SimpleIndexSet::new();
    index_set.insert_sorted(5);
    let (index, inserted) = index_set.insert_sorted(5);
    assert_eq!(index, 0);
    assert!(!inserted);
}

#[test]
fn test_insert_sorted_multiple_values() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    struct SimpleIndexSet {
        items: Vec<i32>,
        hash_builder: TestHasher,
    }

    impl SimpleIndexSet {
        fn new() -> Self {
            Self {
                items: Vec::new(),
                hash_builder: TestHasher,
            }
        }

        fn insert_sorted(&mut self, value: i32) -> (usize, bool) {
            let pos = self.items.binary_search(&value);
            match pos {
                Ok(index) => (index, false),
                Err(index) => {
                    self.items.insert(index, value);
                    (index, true)
                }
            }
        }
    }

    let mut index_set = SimpleIndexSet::new();
    index_set.insert_sorted(3);
    index_set.insert_sorted(1);
    index_set.insert_sorted(2);

    assert_eq!(index_set.items, vec![1, 2, 3]);
    let (index, inserted) = index_set.insert_sorted(2);
    assert_eq!(index, 1);
    assert!(!inserted);
}

