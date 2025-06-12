// Answer 0

#[test]
fn test_into_key_occupied() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let key_two = Rc::new("b");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    assert_eq!(map[&key_one], 10);
    assert!(Rc::strong_count(&key_one) == 2);

    let inside_key: &mut Rc<&str>;

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => {
            inside_key = o.into_key();
        }
    }
    *inside_key = key_two.clone();

    assert_eq!(map[&key_two], 10);
    assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);
}

#[test]
#[should_panic]
fn test_into_key_vacant() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => {
            // Expected to trigger a panic
            panic!();
        },
        RawEntryMut::Occupied(_) => {}
    }
}

#[test]
fn test_into_key_multiple_inserts() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let key_two = Rc::new("b");
    let key_three = Rc::new("c");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);
    map.insert(key_two.clone(), 20);

    assert_eq!(map[&key_one], 10);
    assert_eq!(map[&key_two], 20);

    let inside_key: &mut Rc<&str>;

    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => {
            inside_key = o.into_key();
        }
    }
    *inside_key = key_three.clone();

    assert_eq!(map[&key_three], 10);
    assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_three) == 2);
}

