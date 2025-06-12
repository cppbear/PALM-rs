// Answer 0

#[test]
fn test_iter_mut() {
    use hashbrown::HashTable;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{BuildHasher, Hash};

    struct TestHasher(DefaultHasher);

    impl BuildHasher for TestHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut table: HashTable<i32, TestHasher> = HashTable::new_in(TestHasher(DefaultHasher::new()));
    let hasher = |val: &i32| {
        let mut s = DefaultHasher::new();
        val.hash(&mut s);
        s.finish()
    };

    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.insert_unique(hasher(&3), 3, hasher);

    // Update all values
    for val in table.iter_mut() {
        *val *= 2;
    }

    assert_eq!(table.len(), 3);
    let mut vec: Vec<i32> = Vec::new();

    for val in &table {
        vec.push(*val);
    }

    // The `Iter` iterator produces items in arbitrary order, sort to test
    vec.sort_unstable();
    assert_eq!(vec, [2, 4, 6]);

    assert_eq!(table.len(), 3);
}

