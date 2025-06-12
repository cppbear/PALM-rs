// Answer 0

#[derive(Debug)]
struct Entry<K> {
    key: K,
}

struct RawEntryV1<K> {
    entries: Vec<Entry<K>>,
    index: usize,
}

impl<K> RawEntryV1<K> {
    pub fn new(entries: Vec<Entry<K>>, index: usize) -> Self {
        Self { entries, index }
    }
    
    pub fn index(&self) -> usize {
        self.index
    }
    
    pub fn key_mut(&mut self) -> &mut K {
        let index = self.index();
        &mut self.entries[index].key
    }
}

#[test]
fn test_key_mut() {
    let mut entry1 = Entry { key: 10 };
    let mut entry2 = Entry { key: 20 };
    let mut raw_entry = RawEntryV1::new(vec![entry1, entry2], 0);

    // Modifying key through the mutable reference
    *raw_entry.key_mut() = 15;

    // Verify that the key was modified
    assert_eq!(raw_entry.entries[0].key, 15);
}

#[test]
fn test_key_mut_boundary() {
    let entry1 = Entry { key: 5 };
    let mut raw_entry = RawEntryV1::new(vec![entry1], 0);

    // Modifying key through the mutable reference
    *raw_entry.key_mut() = 8;

    // Verify that the key was modified
    assert_eq!(raw_entry.entries[0].key, 8);
}

