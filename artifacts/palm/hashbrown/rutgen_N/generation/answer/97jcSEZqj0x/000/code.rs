// Answer 0

#[test]
fn test_into_key_value() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_one = Rc::new("a");
    let key_two = Rc::new("a");

    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();
    map.insert(key_one.clone(), 10);
    
    assert_eq!(map[&key_one], 10);
    assert!(Rc::strong_count(&key_one) == 2 && Rc::strong_count(&key_two) == 1);

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
    
    assert_eq!(map[&key_two], 100);
    assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);
}

#[test]
#[should_panic]
fn test_into_key_value_panic_on_vacant() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::rc::Rc;

    let key_three = Rc::new("b");
    
    let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

    match map.raw_entry_mut().from_key(&key_three) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(_) => {}
    }
}

