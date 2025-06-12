// Answer 0

#[derive(Debug)]
struct MockMap<K> {
    entries: Vec<MockEntry<K>>,
}

#[derive(Debug)]
struct MockEntry<K> {
    key: K,
}

struct MockEntryRef<K> {
    map: MockMap<K>,
    index: usize,
}

impl<K> MockEntryRef<K> {
    pub fn key(&self) -> &K {
        &self.map.entries[self.index].key
    }
}

#[test]
fn test_key_retrieval() {
    let entries = vec![MockEntry { key: 1 }, MockEntry { key: 2 }, MockEntry { key: 3 }];
    let map = MockMap { entries };
    let entry_ref = MockEntryRef { map, index: 1 };

    assert_eq!(*entry_ref.key(), 2);
}

#[test]
fn test_key_retrieval_at_bounds() {
    let entries = vec![MockEntry { key: 0 }];
    let map = MockMap { entries };
    
    let entry_ref = MockEntryRef { map, index: 0 };

    assert_eq!(*entry_ref.key(), 0);
}

#[test]
#[should_panic]
fn test_key_retrieval_out_of_bounds() {
    let entries = vec![MockEntry { key: 10 }];
    let map = MockMap { entries };
    
    let entry_ref = MockEntryRef { map, index: 1 };

    // This should panic due to index being out of bounds.
    let _ = entry_ref.key();
}

