// Answer 0

#[derive(Default)]
struct Map {
    indices: Vec<usize>,
}

struct RawEntry {
    map: Map,
}

impl RawEntry {
    pub fn index(&self) -> usize {
        self.map.indices.len()
    }
}

#[test]
fn test_index_empty_map() {
    let entry = RawEntry { map: Map::default() };
    assert_eq!(entry.index(), 0);
}

#[test]
fn test_index_non_empty_map() {
    let mut entry = RawEntry { map: Map::default() };
    entry.map.indices.push(1);
    assert_eq!(entry.index(), 1);
    entry.map.indices.push(2);
    assert_eq!(entry.index(), 2);
}

#[test]
fn test_index_large_map() {
    let mut entry = RawEntry { map: Map::default() };
    for i in 0..1_000_000 {
        entry.map.indices.push(i);
    }
    assert_eq!(entry.index(), 1_000_000);
}

