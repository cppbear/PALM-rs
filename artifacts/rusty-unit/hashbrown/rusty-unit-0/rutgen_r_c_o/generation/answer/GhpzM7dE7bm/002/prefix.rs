// Answer 0

#[test]
fn test_try_insert_occupied_error() {
    let mut map = HashMap::new();
    
    // Insert a value with key 500
    map.try_insert(500, 100).unwrap();
    
    // Try to insert a new value with the same key, which should result in an error
    let result = map.try_insert(500, 200);
    
    // The result should be an Err with an OccupiedError
    match result {
        Err(error) => {
            // Ensure that the error key is the same as the original key
            assert_eq!(error.entry.key(), &500);
            // Ensure that the value passed in the error is 200
            assert_eq!(error.value, 200);
        }
        _ => panic!("Expected an error due to key being occupied."),
    }
}

#[test]
fn test_try_insert_multiple_occurrences() {
    let mut map = HashMap::new();
    
    // Insert multiple key-value pairs
    map.try_insert(250, 300).unwrap();
    map.try_insert(750, 500).unwrap();

    // Try inserting values with the same keys to trigger OccupiedError
    let result1 = map.try_insert(250, 400);
    let result2 = map.try_insert(750, 600);
    
    // Check the first result for key 250
    match result1 {
        Err(error) => {
            assert_eq!(error.entry.key(), &250);
            assert_eq!(error.value, 400);
        }
        _ => panic!("Expected an error due to key 250 being occupied."),
    }

    // Check the second result for key 750
    match result2 {
        Err(error) => {
            assert_eq!(error.entry.key(), &750);
            assert_eq!(error.value, 600);
        }
        _ => panic!("Expected an error due to key 750 being occupied."),
    }
}

#[test]
fn test_try_insert_on_existing_key() {
    let mut map = HashMap::new();
    
    // Insert a key-value pair
    map.try_insert(950, 999).unwrap();
    
    // Try to insert a new value for the existing key
    let result = map.try_insert(950, 750);
    
    // Expect an Err because the key is already occupied
    match result {
        Err(error) => {
            assert_eq!(error.entry.key(), &950);
            assert_eq!(error.value, 750);
        }
        _ => panic!("Expected an error for occupied key."),
    }
}

