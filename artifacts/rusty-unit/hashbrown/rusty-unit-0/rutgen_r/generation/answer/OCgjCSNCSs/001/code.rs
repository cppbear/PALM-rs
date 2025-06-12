// Answer 0

#[test]
fn test_insert_with_hasher_vacant_entry() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    // Define a simple hasher builder
    struct SimpleHasher;

    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    // Creating a HashMap with the SimpleHasher
    let mut map: HashMap<&str, u32, SimpleHasher> = HashMap::with_capacity_and_hasher(10, SimpleHasher);
    
    let key = "a";
    let hash_builder = SimpleHasher;
    let hash = make_hasher(&hash_builder)(&key);

    match map.raw_entry_mut().from_hash(hash, |q| q == &key) {
        RawEntryMut::Occupied(_) => panic!(),
        RawEntryMut::Vacant(v) => {
            let (k, v) = v.insert_with_hasher(hash, key, 100, make_hasher(&hash_builder));
            assert_eq!(*k, "a");
            assert_eq!(*v, 100);
        },
    }
}

#[test]
fn test_insert_with_hasher_existing_entry() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    // Define a simple hasher builder
    struct SimpleHasher;

    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    // Creating a HashMap with the SimpleHasher
    let mut map: HashMap<&str, u32, SimpleHasher> = HashMap::with_capacity_and_hasher(10, SimpleHasher);
    
    let key = "a";
    let hash_builder = SimpleHasher;
    let hash = make_hasher(&hash_builder)(&key);

    // Insert initial value
    let (k, v) = map.raw_entry_mut().from_hash(hash, |q| q == &key).vacant().insert_with_hasher(hash, key, 100, make_hasher(&hash_builder));
    
    assert_eq!(*k, "a");
    assert_eq!(*v, 100);
    
    // Try to insert again, checking panic
    let result = std::panic::catch_unwind(|| {
        map.raw_entry_mut().from_hash(hash, |q| q == &key).occupied().insert_with_hasher(hash, key, 200, make_hasher(&hash_builder));
    });

    assert!(result.is_err(), "Expected panic when inserting with an existing key");
}

fn make_hasher<K, S>(hash_builder: &S) -> impl Fn(&K) -> u64 + '_
where
    K: Hash + ?Sized,
    S: BuildHasher,
{
    move |key: &K| {
        use core::hash::Hasher;
        let mut state = hash_builder.build_hasher();
        key.hash(&mut state);
        state.finish()
    }
}

