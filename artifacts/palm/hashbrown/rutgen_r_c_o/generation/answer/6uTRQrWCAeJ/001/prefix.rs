// Answer 0

#[test]
fn test_key_mut_basic() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let key_two = Rc::new("b");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);
    
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Occupied(mut o) => {
            let key_mut_ref = o.key_mut();
            *key_mut_ref = key_two.clone();
        }
        RawEntryMut::Vacant(_) => panic!(),
    }

    let value = map.get(&key_two).unwrap();
}

#[test]
fn test_key_mut_multiple_references() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("c");
    let key_two = Rc::new("d");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 20);
    
    assert!(Rc::strong_count(&key_one) == 2);
    
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Occupied(mut o) => {
            let key_mut_ref = o.key_mut();
            *key_mut_ref = key_two.clone();
        }
        RawEntryMut::Vacant(_) => panic!(),
    }
    
    assert!(Rc::strong_count(&key_two) == 1);
    let value = map.get(&key_two).unwrap();
}

#[test]
fn test_key_mut_edge_case() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("e");
    let key_two = Rc::new("f");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 30);
    
    assert!(Rc::strong_count(&key_one) == 1);
    
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Occupied(mut o) => {
            let key_mut_ref = o.key_mut();
            *key_mut_ref = key_two.clone();
        }
        RawEntryMut::Vacant(_) => panic!(),
    }

    let value = map.get(&key_two).unwrap();
}

#[test]
#[should_panic]
fn test_key_mut_panic() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("g");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Occupied(_) => panic!(),
        RawEntryMut::Vacant(_) => {
            // Trying to mutate the key of a non-existent entry.
        }
    }
}

