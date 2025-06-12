// Answer 0

#[derive(Default)]
struct Map<K, V> {
    core: Vec<(K, V)>,
}

impl<K, V> Map<K, V> {
    pub fn len(&self) -> usize {
        self.core.len()
    }

    pub fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, K, V>> {
        if index >= self.len() {
            return None;
        }
        Some(IndexedEntry::new(&mut self.core, index))
    }
}

struct IndexedEntry<'a, K, V> {
    entry: &'a mut (K, V),
}

impl<'a, K, V> IndexedEntry<'a, K, V> {
    pub fn new(core: &'a mut Vec<(K, V)>, index: usize) -> Self {
        Self { entry: &mut core[index] }
    }
}

#[test]
fn test_get_index_entry_with_invalid_index() {
    let mut map: Map<i32, i32> = Map::default();
    assert_eq!(map.get_index_entry(0), None); // index == self.len() where self.len() == 0
}

#[test]
fn test_get_index_entry_with_boundary_index() {
    let mut map: Map<i32, i32> = Map {
        core: vec![(1, 2)],
    };
    assert_eq!(map.get_index_entry(1), None); // index == self.len() where self.len() == 1
}

