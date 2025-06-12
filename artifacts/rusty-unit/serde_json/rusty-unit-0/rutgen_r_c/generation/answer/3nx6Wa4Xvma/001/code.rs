// Answer 0

#[test]
fn test_into_mut_with_existing_entry() {
    use serde_json::json;
    use std::collections::BTreeMap;

    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    map.insert("key".to_string(), json!([1, 2]));

    struct TestOccupiedEntry<'a> {
        occupied: OccupiedEntryImpl<'a>,
    }
    
    let occupied_entry = TestOccupiedEntry {
        occupied: map.get_mut("key").unwrap().into(),
    };
    
    let mut occupied = OccupiedEntry { 
        occupied: occupied_entry.occupied 
    };

    // before mutation
    assert_eq!(occupied.get().as_array().unwrap().len(), 2);

    // mutate the entry
    occupied.into_mut().as_array_mut().unwrap().push(json!(3));

    // after mutation
    assert_eq!(occupied.get().as_array().unwrap().len(), 3);
}

#[test]
#[should_panic(expected = "attempt to mutate a vacant entry")]
fn test_into_mut_with_vacant_entry() {
    use serde_json::json;
    use std::collections::BTreeMap;

    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    
    struct TestOccupiedEntry<'a> {
        occupied: OccupiedEntryImpl<'a>,
    }

    // No entry to fetch
    let entry = map.get_mut("non_existent_key");
    
    // This would panic since there is no occupied entry
    let occupied_entry = entry.unwrap(); 
    let mut occupied = OccupiedEntry { 
        occupied: occupied_entry.into() 
    };

    // Attempt mutation on a non-existent entry
    occupied.into_mut();
} 

#[test]
fn test_into_mut_with_edge_case_empty_value() {
    use serde_json::json;
    use std::collections::BTreeMap;

    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    map.insert("empty".to_string(), json!(null));

    struct TestOccupiedEntry<'a> {
        occupied: OccupiedEntryImpl<'a>,
    }

    let occupied_entry = TestOccupiedEntry {
        occupied: map.get_mut("empty").unwrap().into(),
    };
    
    let mut occupied = OccupiedEntry { 
        occupied: occupied_entry.occupied 
    };

    // before mutation
    assert!(occupied.get().is_null());

    // mutate the entry
    occupied.into_mut().as_array_mut().unwrap().push(json!(1));

    // after mutation
    assert!(occupied.get().as_array().unwrap().len(), 1);
} 

#[test]
fn test_into_mut_with_complex_structure() {
    use serde_json::json;
    use std::collections::BTreeMap;

    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    let array_value = json!([1, 2, 3]);
    map.insert("numbers".to_string(), array_value.clone());

    struct TestOccupiedEntry<'a> {
        occupied: OccupiedEntryImpl<'a>,
    }
    
    let occupied_entry = TestOccupiedEntry {
        occupied: map.get_mut("numbers").unwrap().into(),
    };
    
    let mut occupied = OccupiedEntry { 
        occupied: occupied_entry.occupied 
    };

    // before mutation
    assert_eq!(occupied.get(), &array_value);

    // mutate the entry
    occupied.into_mut().as_array_mut().unwrap().push(json!(4));

    // after mutation
    assert_eq!(occupied.get().as_array().unwrap(), &json!([1, 2, 3, 4]));
}

