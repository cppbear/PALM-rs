// Answer 0

#[test]
fn test_insert_key_with_unique_key() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let key_two = Rc::new("b");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    assert_eq!(map[&key_one], 10);
    assert!(Rc::strong_count(&key_one) == 2 && Rc::strong_count(&key_two) == 1);

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let old_key = o.insert_key(key_two.clone());
            assert!(Rc::ptr_eq(&old_key, &key_one));
        }
    }
    assert_eq!(map[&key_two], 10);
    assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);
}

#[test]
fn test_insert_key_with_identical_key() {
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
            let old_key = o.insert_key(key_two.clone());
            assert!(Rc::ptr_eq(&old_key, &key_one));
        }
    }
    assert_eq!(map[&key_two], 10);
    assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);
}

#[test]
#[should_panic]
fn test_insert_key_with_null_key() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one: Option<Rc<&str>> = None;

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(Rc::new("a"), 10);

    match map.raw_entry_mut().from_key(&key_one.as_ref().unwrap()) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(_) => {}
    }
}

