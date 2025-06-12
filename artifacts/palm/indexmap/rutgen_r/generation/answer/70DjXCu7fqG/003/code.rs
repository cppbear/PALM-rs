// Answer 0

#[test]
fn test_get_index_of_multiple_entries() {
    use indexmap::IndexMap;
    use std::hash::{Hash, Hasher};
    
    struct TestKey(u32);
    
    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    
    impl PartialEq for TestKey {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    
    struct TestEquivalent;

    impl<Q: ?Sized> Equivalent<TestKey> for TestEquivalent {
        fn equivalent(&self, key: &TestKey) -> bool {
            key.0 % 2 == 0 // Example condition for equivalence
        }
    }
    
    let mut map = IndexMap::new();
    map.insert(TestKey(1), "one");
    map.insert(TestKey(2), "two");
    map.insert(TestKey(3), "three");

    let found_index = map.get_index_of(&TestKey(2));
    assert_eq!(found_index, Some(1)); // Key 2 exists at index 1

    let not_found_index = map.get_index_of(&TestKey(4));
    assert_eq!(not_found_index, None); // Key 4 does not exist in map
}

#[test]
fn test_get_index_of_empty_map() {
    use indexmap::IndexMap;
    
    let map: IndexMap<u32, &str> = IndexMap::new();
    let index = map.get_index_of(&TestKey(1));
    assert_eq!(index, None); // Key not found, map is empty
}

#[test]
fn test_get_index_of_single_entry() {
    use indexmap::IndexMap;
    
    let mut map = IndexMap::new();
    map.insert(TestKey(1), "one");

    let found_index = map.get_index_of(&TestKey(1));
    assert_eq!(found_index, Some(0)); // Key 1 exists at index 0
}

