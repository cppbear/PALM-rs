// Answer 0

#[test]
fn test_key_mut_valid() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let key_two = Rc::new("a");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    assert_eq!(map[&key_one], 10);
    assert!(Rc::strong_count(&key_one) == 2 && Rc::strong_count(&key_two) == 1);

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let key_mut_ref = o.key_mut(); // Get a mutable reference to the key
            *key_mut_ref = key_two.clone(); // Update the key
        }
    }
    assert_eq!(map[&key_two], 10);
    assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);
}

#[should_panic]
fn test_key_mut_panic_on_empty() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    
    // This will trigger a panic as there are no entries in the map
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!("Expected Occupied entry, found Vacant"),
        RawEntryMut::Occupied(_) => {}
    }
}

