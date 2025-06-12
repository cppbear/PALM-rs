// Answer 0

#[derive(Debug)]
struct Entry<V> {
    value: V,
}

#[derive(Debug)]
struct IndexMap<V> {
    entries: Vec<Entry<V>>,
}

impl<V> IndexMap<V> {
    fn new() -> Self {
        IndexMap { entries: Vec::new() }
    }

    fn push(&mut self, value: V) {
        self.entries.push(Entry { value });
    }

    fn index_mut(&mut self, index: usize) -> &mut V {
        &mut self.entries[index].value
    }
}

#[test]
fn test_index_mut_valid_index() {
    let mut map = IndexMap::new();
    map.push(10);
    map.push(20);

    let value = map.index_mut(1);
    *value = 30;

    assert_eq!(map.entries[1].value, 30);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_index_mut_out_of_bounds() {
    let mut map = IndexMap::new();
    map.push(10);
    
    // This should panic as the index 1 is out of bounds.
    let _ = map.index_mut(1);
}

#[test]
fn test_index_mut_multiple_changes() {
    let mut map = IndexMap::new();
    map.push(1);
    map.push(2);
    map.push(3);

    let value1 = map.index_mut(0);
    let value2 = map.index_mut(1);
    *value1 += 1;
    *value2 *= 2;

    assert_eq!(map.entries[0].value, 2);
    assert_eq!(map.entries[1].value, 4);
}

