// Answer 0

#[derive(Default)]
struct MockMap<V> {
    entries: Vec<MockEntry<V>>,
}

#[derive(Default)]
struct MockEntry<V> {
    value: V,
}

struct IndexedEntry<'a, V> {
    map: &'a mut MockMap<V>,
    index: usize,
}

impl<V> IndexedEntry<'_, V> {
    pub fn get_mut(&mut self) -> &mut V {
        &mut self.map.entries[self.index].value
    }
}

#[test]
fn test_get_mut() {
    let mut map = MockMap {
        entries: vec![MockEntry { value: 42 }],
    };
    let mut entry = IndexedEntry { map: &mut map, index: 0 };
    
    *entry.get_mut() += 1; // increment the value
    assert_eq!(map.entries[0].value, 43);
}

#[test]
fn test_get_mut_multiple_access() {
    let mut map = MockMap {
        entries: vec![MockEntry { value: 100 }],
    };
    let mut entry = IndexedEntry { map: &mut map, index: 0 };
    
    let value_ref = entry.get_mut();
    *value_ref -= 50;
    assert_eq!(*value_ref, 50);
    assert_eq!(map.entries[0].value, 50);
}

#[test]
#[should_panic]
fn test_get_mut_out_of_bounds() {
    let mut map = MockMap {
        entries: vec![],
    };
    let mut entry = IndexedEntry { map: &mut map, index: 0 };
    
    // This should panic since there are no entries
    let _ = entry.get_mut();
}

