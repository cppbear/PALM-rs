// Answer 0

#[test]
fn test_get_key_value_mut_occupied_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("key1");
    let key_two = Rc::new("key2");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    // Checking initial state
    assert_eq!(map[&key_one], 10);
    assert!(Rc::strong_count(&key_one) == 2 && Rc::strong_count(&key_two) == 1);

    // Accessing the RawEntryMut
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!("Expected an occupied entry but found vacant."),
        RawEntryMut::Occupied(mut o) => {
            let (inside_key, inside_value) = o.get_key_value_mut();
            *inside_key = key_two.clone();
            *inside_value = 100;
        }
    }

    // Verifying the changes
    assert_eq!(map[&key_two], 100);
    assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);
}

#[test]
#[should_panic]
fn test_get_key_value_mut_vacant_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("key1");
    
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

    // Attempting to access a vacant entry should panic
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!("This should panic since the entry is vacant."),
        RawEntryMut::Occupied(_) => {}
    }
}

