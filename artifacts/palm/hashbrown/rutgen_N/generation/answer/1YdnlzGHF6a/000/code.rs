// Answer 0

#[test]
fn test_capacity_with_initial_capacity() {
    struct HashTable<T> {
        raw: std::collections::hash_map::HashMap<T, usize>,
    }

    impl<T> HashTable<T> {
        fn with_capacity(capacity: usize) -> Self {
            HashTable {
                raw: std::collections::hash_map::HashMap::with_capacity(capacity),
            }
        }

        fn capacity(&self) -> usize {
            self.raw.capacity()
        }
    }

    let table: HashTable<i32> = HashTable::with_capacity(100);
    assert!(table.capacity() >= 100);
}

#[test]
fn test_capacity_with_zero_capacity() {
    struct HashTable<T> {
        raw: std::collections::hash_map::HashMap<T, usize>,
    }

    impl<T> HashTable<T> {
        fn with_capacity(capacity: usize) -> Self {
            HashTable {
                raw: std::collections::hash_map::HashMap::with_capacity(capacity),
            }
        }

        fn capacity(&self) -> usize {
            self.raw.capacity()
        }
    }

    let table: HashTable<i32> = HashTable::with_capacity(0);
    assert_eq!(table.capacity(), 0);
}

#[test]
fn test_capacity_with_large_initial_capacity() {
    struct HashTable<T> {
        raw: std::collections::hash_map::HashMap<T, usize>,
    }

    impl<T> HashTable<T> {
        fn with_capacity(capacity: usize) -> Self {
            HashTable {
                raw: std::collections::hash_map::HashMap::with_capacity(capacity),
            }
        }

        fn capacity(&self) -> usize {
            self.raw.capacity()
        }
    }

    let table: HashTable<i32> = HashTable::with_capacity(10000);
    assert!(table.capacity() >= 10000);
}

