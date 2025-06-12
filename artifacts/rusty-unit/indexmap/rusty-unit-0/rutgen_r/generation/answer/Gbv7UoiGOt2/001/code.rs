// Answer 0

#[derive(Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
}

#[derive(Debug)]
struct Map<K, V> {
    entries: Vec<Entry<K, V>>,
}

#[derive(Debug)]
struct EntryRef<'a, K, V> {
    map: &'a mut Map<K, V>,
    index: usize,
}

impl<K, V> EntryRef<'_, K, V> {
    pub fn index(&self) -> usize {
        self.index
    }
    
    pub fn into_key_value_mut(self) -> (&'a mut K, &'a mut V) {
        let index = self.index();
        let entry = &mut self.map.entries[index];
        (&mut entry.key, &mut entry.value)
    }
}

#[test]
fn test_into_key_value_mut_valid() {
    // Setup a valid map with some entries
    let mut map = Map {
        entries: vec![
            Entry { key: 1, value: "one" },
            Entry { key: 2, value: "two" },
        ],
    };

    // Create an EntryRef at a valid index
    let entry_ref = EntryRef { map: &mut map, index: 0 };

    // Call into_key_value_mut and check values
    let (key, value) = entry_ref.into_key_value_mut();

    assert_eq!(*key, 1);
    assert_eq!(*value, "one");

    // Modify the key and value
    *key = 10;
    *value = "ten";

    assert_eq!(map.entries[0].key, 10);
    assert_eq!(map.entries[0].value, "ten");
}

#[test]
#[should_panic]
fn test_into_key_value_mut_invalid_index() {
    // Setup a map with one entry
    let mut map = Map {
        entries: vec![Entry { key: 1, value: "one" }],
    };

    // Create an EntryRef for an invalid index
    let entry_ref = EntryRef { map: &mut map, index: 1 }; // Invalid index, should panic here

    // Call into_key_value_mut
    entry_ref.into_key_value_mut();
}

