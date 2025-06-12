// Answer 0

#[test]
fn test_insert_key_with_unique_rc_strings() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("unique_key_1".to_string());
    let key_two = Rc::new("unique_key_2".to_string());

    let mut map: HashMap<Rc<String>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let old_key = o.insert_key(key_two.clone());
        }
    }
}

#[test]
fn test_insert_key_with_empty_string() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("".to_string());
    let key_two = Rc::new("empty".to_string());

    let mut map: HashMap<Rc<String>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let old_key = o.insert_key(key_two.clone());
        }
    }
}

#[test]
fn test_insert_key_with_long_string() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a".repeat(256)); // 256 characters long
    let key_two = Rc::new("long_key".to_string());

    let mut map: HashMap<Rc<String>, u32> = HashMap::new();
    map.insert(key_one.clone(), 20);

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let old_key = o.insert_key(key_two.clone());
        }
    }
}

#[test]
fn test_insert_key_with_multiple_elements() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("key_1".to_string());
    let key_two = Rc::new("key_2".to_string());
    let key_three = Rc::new("key_3".to_string());

    let mut map: HashMap<Rc<String>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);
    map.insert(key_three.clone(), 30);

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let old_key = o.insert_key(key_two.clone());
        }
    }
}

#[test]
#[should_panic]
fn test_insert_key_panic_condition() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a".to_string());
    let key_two = Rc::new("a".to_string());

    let mut map: HashMap<Rc<String>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let old_key = o.insert_key(key_two.clone());
        }
    }
}

