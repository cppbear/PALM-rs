// Answer 0

#[test]
fn test_into_key() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    // Create two Rc keys with the same value
    let key_one = Rc::new("a");
    let key_two = Rc::new("a");

    // Initialize the HashMap
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    // Case where the key_one has been inserted into the map
    assert_eq!(map[&key_one], 10);
    assert!(Rc::strong_count(&key_one) == 2);

    let inside_key: &mut Rc<&str>;

    // Utilize raw entry to mutate the existing key
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!("Expected occupied entry, found vacant."),
        RawEntryMut::Occupied(o) => inside_key = o.into_key(),
    }

    // Change the key to key_two
    *inside_key = key_two.clone();

    // Verify the map now recognizes key_two instead of key_one
    assert_eq!(map[&key_two], 10);
    assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);
}

#[test]
#[should_panic(expected = "Expected occupied entry, found vacant.")]
fn test_into_key_vacant() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    // Initialize an empty HashMap
    let map: HashMap<Rc<&str>, u32> = HashMap::new();

    // Attempt to access a key that doesn't exist
    let key = Rc::new("nonexistent");

    match map.raw_entry_mut().from_key(&key) {
        RawEntryMut::Vacant(_) => panic!("Expected occupied entry, found vacant."),
        RawEntryMut::Occupied(_) => {}
    }
}

