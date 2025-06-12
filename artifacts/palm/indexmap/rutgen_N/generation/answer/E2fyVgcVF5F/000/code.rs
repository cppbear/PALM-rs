// Answer 0

#[derive(Debug)]
struct MockEntry<V> {
    value: V,
}

struct MockMap<V> {
    entries: Vec<MockEntry<V>>,
}

struct Entry<'a, V> {
    map: &'a MockMap<V>,
    index: usize,
}

impl<'a, V> Entry<'a, V> {
    pub fn get(&self) -> &V {
        &self.map.entries[self.index].value
    }
}

#[test]
fn test_get_valid_index() {
    let entries = vec![MockEntry { value: 1 }, MockEntry { value: 2 }];
    let mock_map = MockMap { entries };
    let entry = Entry { map: &mock_map, index: 0 };
    assert_eq!(*entry.get(), 1);
}

#[test]
fn test_get_another_valid_index() {
    let entries = vec![MockEntry { value: 1 }, MockEntry { value: 2 }];
    let mock_map = MockMap { entries };
    let entry = Entry { map: &mock_map, index: 1 };
    assert_eq!(*entry.get(), 2);
}

#[should_panic]
#[test]
fn test_get_invalid_index() {
    let entries = vec![MockEntry { value: 1 }, MockEntry { value: 2 }];
    let mock_map = MockMap { entries };
    let entry = Entry { map: &mock_map, index: 2 }; // This index is out of bounds
    let _ = entry.get();
}

