// Answer 0

#[test]
fn test_key_mut() {
    use core::cell::RefCell;
    use super::IndexMap;

    struct MyKey {
        value: i32,
    }

    struct MyValue {
        data: String,
    }

    let mut index_map = IndexMap::new();
    index_map.insert(MyKey { value: 1 }, MyValue { data: "one".to_string() });

    {
        let mut entry = IndexedEntry::new(&mut index_map, 0);
        let key_mut_ref: &mut MyKey = entry.key_mut();
        key_mut_ref.value = 2; // Modify the key value to test mutability
        assert_eq!(key_mut_ref.value, 2);
        assert_eq!(entry.key().value, 2); // Check if the change is reflected
    }

    // Ensure the indexed entry can retrieve the original key again (in non-mut state)
    {
        let entry = IndexedEntry::new(&mut index_map, 0);
        assert_eq!(entry.key().value, 2); // Check if the key value persisted
    }
}

#[test]
#[should_panic]
fn test_key_mut_panic() {
    use core::cell::RefCell;
    use super::IndexMap;

    struct MyKey {
        value: i32,
    }

    struct MyValue {
        data: String,
    }

    let mut index_map = IndexMap::new();
    
    let mut entry = IndexedEntry::new(&mut index_map, 0);
    let _key_mut_ref: &mut MyKey = entry.key_mut(); // This should panic due to index out of bounds
}

