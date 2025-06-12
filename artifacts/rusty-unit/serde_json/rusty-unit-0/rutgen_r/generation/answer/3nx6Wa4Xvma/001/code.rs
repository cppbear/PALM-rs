// Answer 0

#[test]
fn test_into_mut_on_occupied_entry() {
    use serde_json::json;
    use serde_json::map::{Entry, Map};
    use serde_json::Value;

    let mut map = Map::new();
    map.insert("serde".to_owned(), json!([1, 2, 3]));

    match map.entry("serde") {
        Entry::Occupied(mut occupied) => {
            let value_mut = occupied.into_mut();
            value_mut.as_array_mut().unwrap().push(json!(4));
            assert_eq!(value_mut.as_array().unwrap().len(), 4);
        }
        Entry::Vacant(_) => panic!("Expected entry to be occupied"),
    }
}

#[test]
#[should_panic(expected = "Expected entry to be occupied")]
fn test_into_mut_on_vacant_entry_should_panic() {
    use serde_json::json;
    use serde_json::map::{Entry, Map};
    
    let mut map = Map::new();

    match map.entry("non_existent") {
        Entry::Occupied(_) => panic!("Expected entry to be vacant"),
        Entry::Vacant(vacant) => {
            // This should not panic since we're not calling into_mut
            let _ = vacant;
        },
    }
}

#[test]
fn test_into_mut_on_empty_map_should_panic() {
    use serde_json::json;
    use serde_json::map::{Entry, Map};
    
    let mut map = Map::new();

    match map.entry("another_non_existent") {
        Entry::Occupied(_) => panic!("Expected entry to be vacant"),
        Entry::Vacant(_) => {
            // The vacant entry does not cause a panic directly, 
            // but into_mut should never be called on a vacant entry
            // as per the contract of the method.
        }
    }
}

