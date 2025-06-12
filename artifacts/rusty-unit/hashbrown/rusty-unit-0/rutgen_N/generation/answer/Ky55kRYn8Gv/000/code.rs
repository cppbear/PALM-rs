// Answer 0

#[test]
fn test_get_key_value_mut() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    // Setup the initial key-value pair
    let key_one = Rc::new("a");
    let key_two = Rc::new("a");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    assert_eq!(map[&key_one], 10);
    assert!(Rc::strong_count(&key_one) == 2 && Rc::strong_count(&key_two) == 1);

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let (inside_key, inside_value) = o.get_key_value_mut();
            *inside_key = key_two.clone();
            *inside_value = 100;
        }
    }
    assert_eq!(map[&key_two], 100);
    assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);
}

#[test]
#[should_panic]
fn test_get_key_value_mut_on_non_existent_key() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let key_two = Rc::new("a");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

    // This should panic since the key does not exist
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(_) => {},
    }
}

