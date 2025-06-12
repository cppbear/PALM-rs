// Answer 0

#[test]
fn test_into_key_single_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let key_two = Rc::new("b");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    let inside_key: &mut Rc<&str>;

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => inside_key = o.into_key(),
    }
    *inside_key = key_two.clone();
}

#[test]
fn test_into_key_multiple_entries() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let key_two = Rc::new("b");
    let key_three = Rc::new("c");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);
    map.insert(key_two.clone(), 20);

    let inside_key: &mut Rc<&str>;

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => inside_key = o.into_key(),
    }
    *inside_key = key_three.clone();
}

#[test]
fn test_into_key_update_value() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let key_two = Rc::new("b");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    let inside_key: &mut Rc<&str>;

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => inside_key = o.into_key(),
    }
    *inside_key = key_two.clone();

    assert_eq!(map[&key_two], 10);
}

#[test]
fn test_into_key_edge_case() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("edge_case");
    let key_two = Rc::new("new_key");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    let inside_key: &mut Rc<&str>;

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => inside_key = o.into_key(),
    }
    *inside_key = key_two.clone();
}

