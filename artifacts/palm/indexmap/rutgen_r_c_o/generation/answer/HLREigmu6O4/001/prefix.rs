// Answer 0

#[test]
fn test_replace_full_existing_value() {
    use std::collections::hash_map::RandomState;

    #[derive(Hash, Eq, PartialEq, Clone)]
    struct TestValue(usize);

    let mut index_set = IndexSet {
        map: IndexMap::new(RandomState::new()),
    };

    index_set.insert(TestValue(1));
    index_set.insert(TestValue(2));
    index_set.insert(TestValue(3));

    let result = index_set.replace_full(TestValue(2));
}

#[test]
fn test_replace_full_multiple_replacements() {
    use std::collections::hash_map::RandomState;

    #[derive(Hash, Eq, PartialEq, Clone)]
    struct TestValue(usize);

    let mut index_set = IndexSet {
        map: IndexMap::new(RandomState::new()),
    };

    for i in 1..=5 {
        index_set.insert(TestValue(i));
    }

    let result1 = index_set.replace_full(TestValue(3));
    let result2 = index_set.replace_full(TestValue(1));
    let result3 = index_set.replace_full(TestValue(5));
}

#[test]
fn test_replace_full_existing_entry_return() {
    use std::collections::hash_map::RandomState;

    #[derive(Hash, Eq, PartialEq, Clone)]
    struct TestValue(usize);

    let mut index_set = IndexSet {
        map: IndexMap::new(RandomState::new()),
    };

    index_set.insert(TestValue(42));
    let result = index_set.replace_full(TestValue(42));
}

#[test]
fn test_replace_full_on_duplicate() {
    use std::collections::hash_map::RandomState;

    #[derive(Hash, Eq, PartialEq, Clone)]
    struct TestValue(usize);

    let mut index_set = IndexSet {
        map: IndexMap::new(RandomState::new()),
    };

    index_set.insert(TestValue(10));
    index_set.insert(TestValue(10)); // Duplicate entry

    let result = index_set.replace_full(TestValue(10));
}

#[test]
fn test_replace_full_with_high_value() {
    use std::collections::hash_map::RandomState;

    #[derive(Hash, Eq, PartialEq, Clone)]
    struct TestValue(usize);

    let mut index_set = IndexSet {
        map: IndexMap::new(RandomState::new()),
    };

    index_set.insert(TestValue(42));
    index_set.insert(TestValue(100));
    index_set.insert(TestValue(200));

    let result = index_set.replace_full(TestValue(100));
}

