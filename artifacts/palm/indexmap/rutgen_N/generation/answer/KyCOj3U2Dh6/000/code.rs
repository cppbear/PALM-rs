// Answer 0

#[test]
fn test_new_set() {
    struct IndexMap;
    
    impl IndexMap {
        pub fn new() -> Self {
            IndexMap
        }
    }
    
    struct IndexSet {
        map: IndexMap,
    }
    
    impl IndexSet {
        pub fn new() -> Self {
            IndexSet {
                map: IndexMap::new(),
            }
        }
    }
    
    let set = IndexSet::new();
    assert!(std::ptr::eq(&set.map, &IndexMap::new()) == false); // Ensure a new map is created
}

#[test]
fn test_new_set_integrity() {
    struct IndexMap;
    
    impl IndexMap {
        pub fn new() -> Self {
            IndexMap
        }
    }
    
    struct IndexSet {
        map: IndexMap,
    }
    
    impl IndexSet {
        pub fn new() -> Self {
            IndexSet {
                map: IndexMap::new(),
            }
        }
    }
    
    let set1 = IndexSet::new();
    let set2 = IndexSet::new();
    
    assert!(!std::ptr::eq(&set1.map, &set2.map)); // Ensure that different sets have different maps
}

