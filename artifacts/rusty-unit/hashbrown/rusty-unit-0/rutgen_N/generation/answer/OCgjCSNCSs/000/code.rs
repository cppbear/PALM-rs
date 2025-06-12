// Answer 0

#[test]
fn test_insert_with_hasher_vacant_entry() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();
    let key = "a";
    let hash_builder = map.hasher().clone();
    let hash = make_hasher(&hash_builder)(&key);

    match map.raw_entry_mut().from_hash(hash, |q| q == &key) {
        RawEntryMut::Occupied(_) => panic!(),
        RawEntryMut::Vacant(v) => {
            assert_eq!(
                v.insert_with_hasher(hash, key, 100, make_hasher(&hash_builder)),
                (&mut "a", &mut 100)
            );
        }
    }
    map.extend([("b", 200), ("c", 300), ("d", 400), ("e", 500), ("f", 600)]);
    assert_eq!(map[&"a"], 100);
}

#[test]
#[should_panic]
fn test_insert_with_hasher_occupied_entry() {
    use core::hash::{BuildHasher, Hash};
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();
    let key = "a";
    let hash_builder = map.hasher().clone();
    let hash = make_hasher(&hash_builder)(&key);

    map.insert(key, 100); // Insert the key first

    match map.raw_entry_mut().from_hash(hash, |q| q == &key) {
        RawEntryMut::Occupied(_) => {
            // This should not panic
            panic!(); 
        },
        RawEntryMut::Vacant(v) => {
            v.insert_with_hasher(hash, key, 200, make_hasher(&hash_builder));
        }
    }
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

