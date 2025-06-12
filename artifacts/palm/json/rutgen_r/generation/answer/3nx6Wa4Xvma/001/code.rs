// Answer 0

#[test]
fn test_entry_into_mut_occupied() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Value;

    let mut map = serde_json::Map::new();
    map.insert("serde".to_owned(), json!([1, 2, 3]));

    match map.entry("serde") {
        Entry::Occupied(mut occupied) => {
            let array_ref = occupied.into_mut().as_array_mut().unwrap();
            array_ref.push(json!(4)); // Modifying the array to include a new element.
            assert_eq!(array_ref.len(), 4); // Assert the length of the array after mutation.
        }
        Entry::Vacant(_) => panic!("The entry should be occupied."),
    }
}

#[test]
fn test_entry_into_mut_vacant() {
    use serde_json::json;
    use serde_json::map::Entry;

    let mut map = serde_json::Map::new();

    match map.entry("non_existent") {
        Entry::Occupied(_) => panic!("The entry should be vacant."),
        Entry::Vacant(vacant) => {
            // The entry is vacant, so we can do something with it if needed.
            // For this test, we just check that we correctly identified it as vacant.
            assert_eq!(vacant.key(), "non_existent");
        }
    }
}

