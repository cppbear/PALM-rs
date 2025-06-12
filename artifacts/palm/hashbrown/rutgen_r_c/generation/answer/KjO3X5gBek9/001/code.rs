// Answer 0

#[test]
fn test_insert_hashed_nocheck() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::collections::hash_map::DefaultHasher;

    struct SimpleHasher;
    
    impl BuildHasher for SimpleHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }
    
    let mut map: HashMap<&str, u32> = HashMap::new();
    let key1 = "c";
    let key2 = "d";
    let hash1 = {
        let mut hasher = SimpleHasher.build_hasher();
        key1.hash(&mut hasher);
        hasher.finish()
    };
    let hash2 = {
        let mut hasher = SimpleHasher.build_hasher();
        key2.hash(&mut hasher);
        hasher.finish()
    };

    match map.raw_entry_mut().from_key_hashed_nocheck(hash1, &key1) {
        RawEntryMut::Occupied(_) => panic!("Entry should be vacant"),
        RawEntryMut::Vacant(v) => {
            assert_eq!(v.insert_hashed_nocheck(hash1, key1, 300), (&mut "c", &mut 300));
        }
    }
    
    map.insert(key1, 300); // Insert initial entry to verify it exists

    match map.raw_entry_mut().from_key_hashed_nocheck(hash2, &key2) {
        RawEntryMut::Occupied(_) => panic!("Entry should be vacant"),
        RawEntryMut::Vacant(v) => {
            assert_eq!(v.insert_hashed_nocheck(hash2, key2, 400), (&mut "d", &mut 400));
        }
    }

    assert_eq!(map[&"c"], 300);
    assert_eq!(map[&"d"], 400);
}

#[test]
#[should_panic]
fn test_insert_hashed_nocheck_panic_on_occupied() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::collections::hash_map::DefaultHasher;

    struct SimpleHasher;

    impl BuildHasher for SimpleHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    let key = "e";
    let hash = {
        let mut hasher = SimpleHasher.build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
    };

    map.insert(key, 500); // Insert initial entry to occupy the key

    match map.raw_entry_mut().from_key_hashed_nocheck(hash, &key) {
        RawEntryMut::Occupied(_) => {
            // Should panic when trying to insert_hashed_nocheck on an occupied entry
            let _ = map.raw_entry_mut().from_key_hashed_nocheck(hash, key).insert_hashed_nocheck(hash, key, 600);
        },
        RawEntryMut::Vacant(_) => panic!("Entry should be occupied"),
    }
}

