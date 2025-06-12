// Answer 0

#[test]
fn test_from_key_hashed_nocheck() {
    use indexmap::IndexMap;
    use indexmap::raw::RawEntryMut;
    use std::hash::Hash;

    struct MyKey {
        value: i32,
    }

    impl PartialEq for MyKey {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }

    impl Eq for MyKey {}

    impl Hash for MyKey {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }

    struct MyHashBuilder;

    impl std::hash::BuildHasher for MyHashBuilder {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<MyKey, String, MyHashBuilder> = IndexMap::new();
    map.insert(MyKey { value: 1 }, "One".to_string());
    
    let hash = {
        let key = MyKey { value: 1 };
        let mut hasher = MyHashBuilder.build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
    };
    
    let entry = map.raw_entry_mut().from_key_hashed_nocheck(hash, &MyKey { value: 1 });
    assert!(entry.is_some());
    assert_eq!(entry.unwrap().key(), &MyKey { value: 1 });
}

#[test]
#[should_panic]
fn test_from_key_hashed_nocheck_non_existent() {
    use indexmap::IndexMap;
    use indexmap::raw::RawEntryMut;
    use std::hash::Hash;

    struct MyKey {
        value: i32,
    }

    impl PartialEq for MyKey {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }

    impl Eq for MyKey {}

    impl Hash for MyKey {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }

    struct MyHashBuilder;

    impl std::hash::BuildHasher for MyHashBuilder {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let map: IndexMap<MyKey, String, MyHashBuilder> = IndexMap::new();
    
    let hash = {
        let key = MyKey { value: 2 };
        let mut hasher = MyHashBuilder.build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
    };

    let _entry = map.raw_entry_mut().from_key_hashed_nocheck(hash, &MyKey { value: 2 });
}

