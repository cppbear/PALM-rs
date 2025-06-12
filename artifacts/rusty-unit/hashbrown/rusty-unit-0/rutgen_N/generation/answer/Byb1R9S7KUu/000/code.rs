// Answer 0

#[test]
fn test_insert_entry() {
    struct TestHasher;

    impl TestHasher {
        fn hash(value: &u64) -> u64 {
            *value
        }
    }

    struct TestTable<T> {
        entries: std::collections::HashMap<u64, T>,
    }

    impl<T> TestTable<T> {
        fn new() -> Self {
            TestTable {
                entries: std::collections::HashMap::new(),
            }
        }

        unsafe fn insert(&mut self, hash: u64, value: T, _: impl Fn(&T) -> u64) -> &mut T {
            self.entries.insert(hash, value);
            self.entries.get_mut(&hash).unwrap()
        }

        pub fn insert_entry(&mut self, hash: u64, value: T, hasher: impl Fn(&T) -> u64) -> &mut T {
            unsafe { self.insert(hash, value, hasher) }
        }
    }

    let mut table = TestTable::new();
    let hash_value = 42;
    let value = 100;

    let entry = table.insert_entry(hash_value, value, TestHasher::hash);
    assert_eq!(*entry, 100);
    assert_eq!(table.entries.len(), 1);
    assert!(table.entries.contains_key(&hash_value));
}

