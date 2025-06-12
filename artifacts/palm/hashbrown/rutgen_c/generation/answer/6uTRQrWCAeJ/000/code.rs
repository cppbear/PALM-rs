// Answer 0

#[test]
fn test_key_mut() {
    use crate::hash_map::HashMap;
    use std::rc::Rc;

    // Create two Rc<String> keys
    let key_one = Rc::new("a");
    let key_two = Rc::new("a");

    // Initialize the HashMap
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    // Assert initial conditions
    assert_eq!(map[&key_one], 10);
    assert!(Rc::strong_count(&key_one) == 2 && Rc::strong_count(&key_two) == 1);

    // Simulate RawEntryMut::Occupied
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            // Change the key using key_mut
            *o.key_mut() = key_two.clone();
        }
    }

    // Assert the new conditions
    assert_eq!(map[&key_two], 10);
    assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);
}

