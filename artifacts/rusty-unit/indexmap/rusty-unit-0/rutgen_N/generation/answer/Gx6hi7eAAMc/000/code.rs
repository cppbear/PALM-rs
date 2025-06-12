// Answer 0

#[derive(Debug)]
struct Entry<'a, V> {
    map: &'a mut IndexMap<V>,
    index: usize,
}

#[derive(Debug)]
struct IndexMap<V> {
    entries: Vec<EntryValue<V>>,
}

#[derive(Debug)]
struct EntryValue<V> {
    value: V,
}

impl<'a, V> Entry<'a, V> {
    pub fn into_mut(self) -> &'a mut V {
        &mut self.map.entries[self.index].value
    }
}

#[test]
fn test_into_mut() {
    let mut map = IndexMap {
        entries: vec![EntryValue { value: 10 }, EntryValue { value: 20 }],
    };
    
    let entry = Entry { map: &mut map, index: 0 };
    let value_mut = entry.into_mut();
    
    *value_mut += 5; // mutate the value

    assert_eq!(map.entries[0].value, 15);
}

#[test]
fn test_into_mut_boundary_condition() {
    let mut map = IndexMap {
        entries: vec![EntryValue { value: 0 }],
    };
    
    let entry = Entry { map: &mut map, index: 0 };
    let value_mut = entry.into_mut();
    
    *value_mut = 100; // set the value directly

    assert_eq!(map.entries[0].value, 100);
}

