// Answer 0

#[test]
fn test_key_mut() {
    struct Entry<K> {
        key: K,
    }

    struct MapEntry<K> {
        entries: Vec<Entry<K>>,
        index: usize,
    }

    impl<K> MapEntry<K> {
        pub(crate) fn new(entries: Vec<Entry<K>>, index: usize) -> Self {
            MapEntry { entries, index }
        }

        pub(crate) fn key_mut(&mut self) -> &mut K {
            let index = self.index;
            &mut self.entries[index].key
        }
    }

    let mut entry = MapEntry::new(vec![Entry { key: 1 }, Entry { key: 2 }], 0);
    let key_mut = entry.key_mut();
    *key_mut = 3;
    assert_eq!(entry.entries[0].key, 3);
}

#[test]
fn test_key_mut_boundary() {
    struct Entry<K> {
        key: K,
    }

    struct MapEntry<K> {
        entries: Vec<Entry<K>>,
        index: usize,
    }

    impl<K> MapEntry<K> {
        pub(crate) fn new(entries: Vec<Entry<K>>, index: usize) -> Self {
            MapEntry { entries, index }
        }

        pub(crate) fn key_mut(&mut self) -> &mut K {
            let index = self.index;
            &mut self.entries[index].key
        }
    }

    let mut entry = MapEntry::new(vec![Entry { key: 1 }], 0);
    let key_mut = entry.key_mut();
    *key_mut = 10;
    assert_eq!(entry.entries[0].key, 10);
}

