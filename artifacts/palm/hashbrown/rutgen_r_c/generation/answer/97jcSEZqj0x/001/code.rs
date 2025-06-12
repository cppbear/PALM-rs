// Answer 0

#[test]
fn test_into_key_value() {
    use std::rc::Rc;
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    // Create a new HashMap to test
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

    // Prepare keys and insert an initial value
    let key_one = Rc::new("a");
    let value_one = 10;
    map.insert(key_one.clone(), value_one);

    assert_eq!(map[&key_one], value_one);

    // Define a mutable reference to hold the key and value
    let (inside_key, inside_value): (&mut Rc<&str>, &mut u32);

    // Obtain a RawEntryMut using the existing key
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!("Expected an occupied entry"),
        RawEntryMut::Occupied(o) => {
            // Call into_key_value to get mutable references to key and value
            let tuple = o.into_key_value();
            inside_key = tuple.0;
            inside_value = tuple.1;
        }
    }

    // Modify the key and value through the mutable references
    let key_two = Rc::new("b");
    *inside_key = key_two.clone();
    *inside_value = 100;

    // Check that the map reflects the changes
    assert_eq!(map[&key_two], 100);
    assert_eq!(Rc::strong_count(&inside_key), 1);
    assert_eq!(Rc::strong_count(&inside_value), 1);
}

#[test]
#[should_panic]
fn test_into_key_value_with_non_existent_key() {
    use std::rc::Rc;
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    // Create a new HashMap to test
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

    // Attempt to access a key that does not exist
    let non_existent_key = Rc::new("non_existent");

    match map.raw_entry_mut().from_key(&non_existent_key) {
        RawEntryMut::Vacant(_) => {
            // We expect this to happen; panic if we get occupied
            panic!("Expected a vacant entry for non-existent key");
        }
        RawEntryMut::Occupied(_) => {
            // Should not reach here; panic if we get occupied
            panic!("Expected vacuum but found occupied entry");
        }
    }
}

