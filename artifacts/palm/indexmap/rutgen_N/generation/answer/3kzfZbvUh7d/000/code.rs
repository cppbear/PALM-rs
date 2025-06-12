// Answer 0

#[derive(Debug)]
struct RawEntryBuilderMut<'a, K, V, S> {
    map: &'a mut IndexMap<K, V, S>,
}

#[derive(Debug)]
struct IndexMap<K, V, S> {
    // Implement minimal fields for the sake of this test
    data: std::collections::HashMap<K, V>, // Assuming S is a type like std::hash::BuildHasher
}

impl<K, V, S> IndexMap<K, V, S> {
    fn new() -> Self {
        IndexMap {
            data: std::collections::HashMap::new(),
        }
    }

    fn raw_entry_mut_v1(&mut self) -> RawEntryBuilderMut<'_, K, V, S> {
        RawEntryBuilderMut { map: self }
    }
}

#[test]
fn test_raw_entry_mut_v1() {
    let mut index_map: IndexMap<i32, String, std::collections::hash_map::RandomState> = IndexMap::new();
    let entry_builder = index_map.raw_entry_mut_v1();
    assert!(std::ptr::eq(entry_builder.map, &mut index_map));
}

#[test]
fn test_raw_entry_mut_v1_empty_map() {
    let mut index_map: IndexMap<String, i32, std::collections::hash_map::RandomState> = IndexMap::new();
    let entry_builder = index_map.raw_entry_mut_v1();
    assert!(std::ptr::eq(entry_builder.map, &mut index_map));
}

