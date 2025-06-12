// Answer 0

#[test]
fn test_search_return_none_when_no_match() {
    struct MockTable;
    
    impl MockTable {
        fn get(&self, _hash: u64, _is_match: &dyn Fn(&u64) -> bool) -> Option<(&u64, &u64)> {
            None
        }
    }
    
    struct MockMap {
        table: MockTable,
    }
    
    struct HashMap<K, V> {
        map: MockMap,
    }
    
    let map = MockMap { table: MockTable };
    let hash_map: HashMap<u64, u64> = HashMap { map };
    
    let result = hash_map.search(42, |k| *k == 1);
    assert_eq!(result, None);
}

#[test]
fn test_search_return_none_with_always_false_match() {
    struct MockTable;
    
    impl MockTable {
        fn get(&self, _hash: u64, _is_match: &dyn Fn(&u64) -> bool) -> Option<(&u64, &u64)> {
            None
        }
    }
    
    struct MockMap {
        table: MockTable,
    }
    
    struct HashMap<K, V> {
        map: MockMap,
    }
    
    let map = MockMap { table: MockTable };
    let hash_map: HashMap<u64, u64> = HashMap { map };
    
    let result = hash_map.search(42, |_| false);
    assert_eq!(result, None);
}

