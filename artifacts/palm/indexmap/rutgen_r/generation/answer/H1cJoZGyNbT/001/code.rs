// Answer 0

#[derive(Debug)]
struct Entry<K> {
    key: K,
}

#[derive(Debug)]
struct Map<K> {
    entries: Vec<Entry<K>>,
}

#[derive(Debug)]
struct EntryRef<K> {
    map: Map<K>,
    index: usize,
}

impl<K> EntryRef<K> {
    pub fn key(&self) -> &K {
        &self.map.entries[self.index].key
    }
}

#[test]
fn test_key_valid_index() {
    let entry1 = Entry { key: "key1" };
    let entry2 = Entry { key: "key2" };
    let map = Map {
        entries: vec![entry1, entry2],
    };
    let entry_ref = EntryRef { map, index: 0 };
    
    assert_eq!(entry_ref.key(), &"key1");
}

#[test]
fn test_key_boundaries() {
    let entry1 = Entry { key: "start" };
    let entry2 = Entry { key: "middle" };
    let entry3 = Entry { key: "end" };
    
    let map = Map {
        entries: vec![entry1, entry2, entry3],
    };

    let entry_ref_start = EntryRef { map: map.clone(), index: 0 };
    let entry_ref_middle = EntryRef { map: map.clone(), index: 1 };
    let entry_ref_end = EntryRef { map: map.clone(), index: 2 };
    
    assert_eq!(entry_ref_start.key(), &"start");
    assert_eq!(entry_ref_middle.key(), &"middle");
    assert_eq!(entry_ref_end.key(), &"end");
}

#[should_panic]
#[test]
fn test_key_out_of_bounds() {
    let map = Map {
        entries: vec![Entry { key: "only_key" }],
    };
    let entry_ref = EntryRef { map, index: 1 };
    
    entry_ref.key(); // This should panic due to out-of-bounds index.
}

