// Answer 0

#[test]
fn test_key_mut_occupied_entry() {
    struct MockEntries<K, V> {
        key: K,
        value: V,
    }

    impl<'a, K: Copy, V> OccupiedEntry<'a, K, V> {
        pub fn create(entries: &'a mut [MockEntries<K, V>], index: usize) -> Self {
            Self::new(entries, hash_table::OccupiedEntry::new(index))
        }
    }

    let mut entries = vec![MockEntries { key: 42, value: "value1" }];
    let index = 0;
    let mut occupied_entry = OccupiedEntry::create(&mut entries, index);
    
    let key_mut = occupied_entry.key_mut();
    *key_mut = 100;

    assert_eq!(*key_mut, 100);
}

#[test]
fn test_key_mut_vacant_entry() {
    struct MockEntries<K, V> {
        key: K,
        value: V,
    }

    struct MutableEntryKeyMock<'a, K, V> {
        entry: Entry<'a, K, V>,
    }

    impl<'a, K: Copy, V> MutableEntryKeyMock<'a, K, V> {
        pub fn new_vacant(key: K) -> Self {
            let vacant_entry = VacantEntry {
                map: RefMut::new(),
                hash: HashValue::default(),
                key,
            };
            Self { entry: Entry::Vacant(vacant_entry) }
        }
    }

    let mut entry = MutableEntryKeyMock::new_vacant(42);
    
    let key_mut = entry.entry.key_mut();
    *key_mut = 100;

    assert_eq!(*key_mut, 100);
}

