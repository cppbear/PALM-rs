// Answer 0

#[test]
fn test_from_key_occupied() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("a", 50);
    map.insert("b", 100);
    
    let key = "a";
    let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_key(&key);
    
    match entry {
        RawEntryMut::Occupied(mut occupied) => {
            assert_eq!(occupied.get(), &50);
            occupied.insert(key, 100);
            assert_eq!(map[&"a"], 100);
        }
        _ => panic!("Expected Occupied entry"),
    }
}

#[test]
fn test_from_key_vacant() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    
    let mut map: HashMap<&str, u32> = HashMap::new();
    
    let key = "c";
    let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_key(&key);
    
    match entry {
        RawEntryMut::Vacant(mut vacant) => {
            vacant.insert(key, 200);
            assert_eq!(map[&"c"], 200);
        }
        _ => panic!("Expected Vacant entry"),
    }
}

#[test]
fn test_from_key_empty_map() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    
    let mut map: HashMap<&str, u32> = HashMap::new();
    
    let key = "test_key";
    let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_key(&key);
    
    match entry {
        RawEntryMut::Vacant(mut vacant) => {
            vacant.insert(key, 50);
            assert_eq!(map[&"test_key"], 50);
        }
        _ => panic!("Expected Vacant entry"),
    }
}

