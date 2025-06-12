// Answer 0

#[test]
fn test_iter_empty_map() {
    struct TestMap<K, V> {
        core: IndexMapCore<K, V>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            TestMap {
                core: IndexMapCore {
                    indices: Indices::new(),  // Note: We need to assume Indices has a new method
                    entries: Entries::new(),   // Note: We are assuming Entries has a new method
                },
            }
        }
        
        fn as_entries(&self) -> &[Bucket<K, V>] {
            &self.core.entries
        }
        
        fn iter(&self) -> Iter<'_, K, V> {
            Iter::new(self.as_entries())
        }
    }
    
    let map: TestMap<i32, i32> = TestMap::new();
    let mut iter = map.iter();
    
    assert!(iter.iter.next().is_none());  // Ensure that the iterator over an empty map returns none
}

#[test]
fn test_iter_single_entry() {
    struct TestMap<K, V> {
        core: IndexMapCore<K, V>,
    }

    impl<K, V> TestMap<K, V> {
        fn new(entries: Vec<Bucket<K, V>>) -> Self {
            TestMap {
                core: IndexMapCore {
                    indices: Indices::with_capacity(entries.len()), // Assuming we can initialize like this
                    entries: Entries::from(entries), // Assuming we have a method to create Entries from a vector
                },
            }
        }
        
        fn as_entries(&self) -> &[Bucket<K, V>] {
            &self.core.entries
        }
        
        fn iter(&self) -> Iter<'_, K, V> {
            Iter::new(self.as_entries())
        }
    }

    let bucket = Bucket { hash: HashValue::new(1), key: 1, value: 10 };
    let map = TestMap::new(vec![bucket]);
    let mut iter = map.iter();
    
    assert!(iter.iter.next().is_some());  // Ensure iterator returns some value
    assert!(iter.iter.next().is_none());  // Ensure there is no more value after the first
}

#[test]
fn test_iter_multiple_entries() {
    struct TestMap<K, V> {
        core: IndexMapCore<K, V>,
    }

    impl<K, V> TestMap<K, V> {
        fn new(entries: Vec<Bucket<K, V>>) -> Self {
            TestMap {
                core: IndexMapCore {
                    indices: Indices::with_capacity(entries.len()), // Assuming we can initialize like this
                    entries: Entries::from(entries), // Assuming we have a method to create Entries from a vector
                },
            }
        }
        
        fn as_entries(&self) -> &[Bucket<K, V>] {
            &self.core.entries
        }
        
        fn iter(&self) -> Iter<'_, K, V> {
            Iter::new(self.as_entries())
        }
    }

    let buckets = vec![
        Bucket { hash: HashValue::new(1), key: 1, value: 10 },
        Bucket { hash: HashValue::new(2), key: 2, value: 20 },
    ];
    let map = TestMap::new(buckets);
    let mut iter = map.iter();
    
    assert!(iter.iter.next().is_some());
    assert!(iter.iter.next().is_some());
    assert!(iter.iter.next().is_none());
}

