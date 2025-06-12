// Answer 0

#[test]
fn test_from_key_hashed_nocheck_occupied() {
    use crate::hash_map::{HashMap, RawEntryMut};
    use crate::hash_map::DefaultHashBuilder;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("a", 100);
    
    let key = "a";
    let hash: u64 = 12345; // a constant hash value for the test
    
    let entry: RawEntryMut<&str, u32, DefaultHashBuilder> = map.raw_entry_mut().from_key_hashed_nocheck(hash, &key);
    
    match entry {
        RawEntryMut::Occupied(mut occupied_entry) => {
            let value = occupied_entry.get(); // retrieve existing value
            assert_eq!(*value, 100);
        }
        RawEntryMut::Vacant(_) => {
            panic!("Expected an occupied entry for key 'a'");
        }
    }
}

#[test]
fn test_from_key_hashed_nocheck_vacant() {
    use crate::hash_map::{HashMap, RawEntryMut};
    use crate::hash_map::DefaultHashBuilder;

    let mut map: HashMap<&str, u32> = HashMap::new();
    
    let key = "b"; // a key that is not in the map
    let hash: u64 = 54321; // a constant hash value for the test
    
    let entry: RawEntryMut<&str, u32, DefaultHashBuilder> = map.raw_entry_mut().from_key_hashed_nocheck(hash, &key);
    
    match entry {
        RawEntryMut::Vacant(vacant_entry) => {
            vacant_entry.insert(key, 200);
            assert_eq!(map[&"b"], 200);
        }
        RawEntryMut::Occupied(_) => {
            panic!("Expected a vacant entry for key 'b'");
        }
    }
}

#[test]
#[should_panic]
fn test_from_key_hashed_nocheck_invalid_key() {
    use crate::hash_map::{HashMap, RawEntryMut};
    use crate::hash_map::DefaultHashBuilder;

    struct InvalidKey;

    impl Equivalent<&str> for InvalidKey {
        fn equivalent(&self, _: &&str) -> bool {
            false // always returns false, simulating an invalid key
        }
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("a", 100);

    let key = InvalidKey;
    let hash: u64 = 98765; // a constant hash value for the test
    
    // This should panic because the key equivalent function will not match
    let entry: RawEntryMut<InvalidKey, u32, DefaultHashBuilder> = map.raw_entry_mut().from_key_hashed_nocheck(hash, &key);
    // Accessing entry should lead to a panic as the key does not exist
    match entry {
        RawEntryMut::Occupied(_) => {}
        RawEntryMut::Vacant(_) => panic!("Expected an occupied entry but got vacant"),
    }
}

