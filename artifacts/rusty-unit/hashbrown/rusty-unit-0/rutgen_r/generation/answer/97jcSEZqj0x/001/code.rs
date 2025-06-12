// Answer 0

#[test]
fn test_into_key_value_occupied_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let key_two = Rc::new("b");

    // Create and initialize the HashMap
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);

    assert_eq!(map[&key_one], 10);
    assert!(Rc::strong_count(&key_one) == 2);

    let inside_key: &mut Rc<&str>;
    let inside_value: &mut u32;
    
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => panic!("The entry should be occupied"),
        RawEntryMut::Occupied(o) => {
            let tuple = o.into_key_value();
            inside_key = tuple.0;
            inside_value = tuple.1;
        }
    }
    
    *inside_key = key_two.clone();
    *inside_value = 100;
    
    assert_eq!(map[&key_two], 100);
    assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);
}

#[test]
#[should_panic(expected = "The entry should be occupied")]
fn test_into_key_value_vacant_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let key_two = Rc::new("b");

    // Create and initialize the HashMap
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    // No entry for key_one, which will lead to the RawEntryMut being vacant

    let inside_key: &mut Rc<&str>;
    let inside_value: &mut u32;
    
    match map.raw_entry_mut().from_key(&key_one) {
        RawEntryMut::Vacant(_) => {
            panic!("The entry should be occupied");
        },
        RawEntryMut::Occupied(o) => {
            let tuple = o.into_key_value();
            inside_key = tuple.0;
            inside_value = tuple.1;
        }
    }
}

