// Answer 0

#[test]
fn test_get_key_value_mut_existing_key() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let key_two = Rc::new("b");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let (inside_key, inside_value) = o.get_key_value_mut();
            *inside_key = key_two.clone();
            *inside_value = 100;
        }
    }
}

#[test]
#[should_panic]
fn test_get_key_value_mut_non_existing_key() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key = Rc::new("c");
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

    match map.raw_entry_mut().from_key(&key) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(_) => {},
    }
}

#[test]
fn test_get_key_value_mut_multiple_keys() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("d");
    let key_two = Rc::new("e");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 20);
    map.insert(key_two.clone(), 30);

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let (inside_key, inside_value) = o.get_key_value_mut();
            *inside_key = Rc::new("f");
            *inside_value = 150;
        }
    }
}

#[test]
#[should_panic]
fn test_get_key_value_mut_invalid_key() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("g");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(Rc::new("h"), 40);

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(_) => {},
    }
}

#[test]
fn test_get_key_value_mut_unique_keys() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("i");
    let key_two = Rc::new("j");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 50);

    let strong_count_one_before = Rc::strong_count(&key_one);
    let strong_count_two_before = Rc::strong_count(&key_two);

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            let (inside_key, inside_value) = o.get_key_value_mut();
            *inside_key = key_two.clone();
            *inside_value = 200;
        }
    }

    assert_eq!(Rc::strong_count(&key_one), strong_count_one_before - 1);
    assert_eq!(Rc::strong_count(&key_two), strong_count_two_before + 1);
}

