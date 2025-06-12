// Answer 0

#[derive(Debug)]
struct Entry<'a, K> {
    index: usize,
    entries: &'a mut Vec<MapEntry<K>>,
}

#[derive(Debug)]
struct MapEntry<K> {
    key: K,
}

impl<'a, K> Entry<'a, K> {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn into_key(self) -> &'a mut K {
        let index = self.index();
        &mut self.entries[index].key
    }
}

#[test]
fn test_into_key_mutable_reference() {
    let mut entries = vec![MapEntry { key: 1 }, MapEntry { key: 2 }];
    let entry = Entry { index: 0, entries: &mut entries };

    let key_ref = entry.into_key();
    *key_ref += 1; // This modifies the original key

    assert_eq!(entries[0].key, 2);
}

#[test]
fn test_into_key_boundary() {
    let mut entries = vec![MapEntry { key: 0 }];
    let entry = Entry { index: 0, entries: &mut entries };

    let key_ref = entry.into_key();
    *key_ref += 100; // Test modification at boundaries

    assert_eq!(entries[0].key, 100);
}

