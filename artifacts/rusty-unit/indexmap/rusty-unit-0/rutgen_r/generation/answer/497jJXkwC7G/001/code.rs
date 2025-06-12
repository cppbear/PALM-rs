// Answer 0

#[test]
fn test_or_insert_vacant() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{BuildHasher, Hash};

    struct MyBuildHasher;

    impl BuildHasher for MyBuildHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    struct VacantEntry<'a, K, V> {
        key: K,
        value: V,
        _marker: std::marker::PhantomData<&'a (K, V)>,
    }

    struct RawEntryVacant<'a, K, V> {
        entry: Option<VacantEntry<'a, K, V>>,
    }

    impl<'a, K, V> RawEntryVacant<'a, K, V> 
    where
        K: Hash,
    {
        fn insert(self, key: K, value: V) -> (&'a mut K, &'a mut V) {
            let key_ref = &mut key;
            let value_ref = &mut value;
            self.entry = Some(VacantEntry {
                key,
                value,
                _marker: std::marker::PhantomData,
            });
            (key_ref, value_ref)
        }
    }

    enum RawEntry<'a, K, V> {
        Occupied(VacantEntry<'a, K, V>),
        Vacant(RawEntryVacant<'a, K, V>),
    }

    impl<'a, K, V> RawEntry<'a, K, V> 
    where
        K: Hash,
    {
        pub fn or_insert(self, default_key: K, default_value: V) -> (&'a mut K, &'a mut V) {
            match self {
                Self::Occupied(entry) => {
                    let key = &mut entry.key;
                    let value = &mut entry.value;
                    (key, value)
                }
                Self::Vacant(entry) => entry.insert(default_key, default_value),
            }
        }
    }

    let vacant_entry: RawEntry<i32, &str> = RawEntry::Vacant(RawEntryVacant { entry: None });
    let (key_ref, value_ref) = vacant_entry.or_insert(1, "value1");

    assert_eq!(*key_ref, 1);
    assert_eq!(*value_ref, "value1");
}

#[test]
#[should_panic]
fn test_or_insert_panic_on_occupied() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{BuildHasher, Hash};

    struct MyBuildHasher;

    impl BuildHasher for MyBuildHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    struct VacantEntry<'a, K, V> {
        key: K,
        value: V,
        _marker: std::marker::PhantomData<&'a (K, V)>,
    }

    struct RawEntryVacant<'a, K, V> {
        entry: Option<VacantEntry<'a, K, V>>,
    }

    enum RawEntry<'a, K, V> {
        Occupied(VacantEntry<'a, K, V>),
        Vacant(RawEntryVacant<'a, K, V>),
    }

    impl<'a, K, V> RawEntry<'a, K, V> 
    where
        K: Hash,
    {
        pub fn or_insert(self, default_key: K, default_value: V) -> (&'a mut K, &'a mut V) {
            match self {
                Self::Occupied(entry) => {
                    let key = &mut entry.key;
                    let value = &mut entry.value;
                    (key, value)
                }
                Self::Vacant(entry) => entry.insert(default_key, default_value),
            }
        }
    }

    let occupied_entry = RawEntry::Occupied(VacantEntry {
        key: 2,
        value: "value2",
        _marker: std::marker::PhantomData,
    });
    
    // This should panic since the entry is occupied
    let _ = occupied_entry.or_insert(3, "value3");
}

