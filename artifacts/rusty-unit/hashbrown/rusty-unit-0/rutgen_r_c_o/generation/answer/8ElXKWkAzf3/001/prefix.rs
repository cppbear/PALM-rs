// Answer 0

#[test]
fn test_get_key_value_occupied() {
    let mut map: HashMap<&'static str, u32> = [("a", 100), ("b", 200)].into();
    
    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Occupied(o) => {
            o.get_key_value();
        },
        _ => panic!(),
    }
}

#[test]
fn test_get_key_value_occupied_b() {
    let mut map: HashMap<&'static str, u32> = [("a", 100), ("b", 200)].into();
    
    match map.raw_entry_mut().from_key(&"b") {
        RawEntryMut::Occupied(o) => {
            o.get_key_value();
        },
        _ => panic!(),
    }
}

#[test]
#[should_panic]
fn test_get_key_value_vacant() {
    let mut map: HashMap<&'static str, u32> = [("a", 100), ("b", 200)].into();
    
    match map.raw_entry_mut().from_key(&"c") {
        RawEntryMut::Occupied(_) => {},
        _ => panic!(),
    }
}

