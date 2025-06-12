// Answer 0

#[test]
fn test_key_mut_valid_range() {
    let mut index_map: IndexMap<i32, String> = IndexMap::new();
    index_map.insert(0, String::from("Value0"));
    let mut entry = IndexedEntry::new(&mut index_map, 0);
    let key_mut = entry.key_mut();
}

#[test]
fn test_key_mut_mid_range() {
    let mut index_map: IndexMap<i32, String> = IndexMap::new();
    index_map.insert(50, String::from("Value50"));
    let mut entry = IndexedEntry::new(&mut index_map, 50);
    let key_mut = entry.key_mut();
}

#[test]
fn test_key_mut_edge_zero() {
    let mut index_map: IndexMap<i32, String> = IndexMap::new();
    index_map.insert(0, String::from("ValueZero"));
    let mut entry = IndexedEntry::new(&mut index_map, 0);
    let key_mut = entry.key_mut();
}

#[test]
fn test_key_mut_edge_hundred() {
    let mut index_map: IndexMap<i32, String> = IndexMap::new();
    index_map.insert(100, String::from("ValueHundred"));
    let mut entry = IndexedEntry::new(&mut index_map, 100);
    let key_mut = entry.key_mut();
}

#[should_panic]
fn test_key_mut_out_of_bounds_negative() {
    let mut index_map: IndexMap<i32, String> = IndexMap::new();
    index_map.insert(1, String::from("Value1"));
    let mut entry = IndexedEntry::new(&mut index_map, usize::MAX);
    let key_mut = entry.key_mut(); // This should panic
}

#[should_panic]
fn test_key_mut_out_of_bounds_exceeding() {
    let mut index_map: IndexMap<i32, String> = IndexMap::new();
    index_map.insert(1, String::from("Value1"));
    let mut entry = IndexedEntry::new(&mut index_map, 2);
    let key_mut = entry.key_mut(); // This should panic
}

