// Answer 0

#[test]
fn test_into_key_value_with_valid_input() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let key_two = Rc::new("b");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    let inside_key: &mut Rc<&str>;
    let inside_value: &mut u32;
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => {
            let tuple = o.into_key_value();
            inside_key = tuple.0;
            inside_value = tuple.1;
        }
    }

    *inside_key = key_two.clone();
    *inside_value = 100;
}

#[test]
fn test_into_key_value_change_key() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("c");
    let key_two = Rc::new("d");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 20);

    let inside_key: &mut Rc<&str>;
    let inside_value: &mut u32;
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => {
            let tuple = o.into_key_value();
            inside_key = tuple.0;
            inside_value = tuple.1;
        }
    }

    *inside_key = key_two.clone();
    *inside_value = 200;
}

#[test]
fn test_into_key_value_with_edge_values() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_edge = Rc::new("z");
    let value_edge = Rc::new("a");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_edge.clone(), 30);

    let inside_key: &mut Rc<&str>;
    let inside_value: &mut u32;
    match map.raw_entry_mut().from_key(&key_edge) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => {
            let tuple = o.into_key_value();
            inside_key = tuple.0;
            inside_value = tuple.1;
        }
    }

    *inside_key = value_edge.clone();
    *inside_value = 300;
}

#[test]
#[should_panic]
fn test_into_key_value_with_non_existent_key() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_non_existent = Rc::new("x");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    
    let inside_key: &mut Rc<&str>;
    let inside_value: &mut u32;
    match map.raw_entry_mut().from_key(&key_non_existent) {
        RawEntryMut::Vacant(_) => {}
        RawEntryMut::Occupied(o) => {
            let tuple = o.into_key_value();
            inside_key = tuple.0;
            inside_value = tuple.1;
        }
    }
}

