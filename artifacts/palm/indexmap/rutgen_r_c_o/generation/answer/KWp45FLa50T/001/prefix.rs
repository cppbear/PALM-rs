// Answer 0

#[test]
fn test_remove_entry_existing_key() {
    let mut map = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let _ = map.remove_entry(&1);
}

#[test]
fn test_remove_entry_non_existing_key() {
    let mut map = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let _ = map.remove_entry(&3);
}

#[test]
fn test_remove_entry_edge_case_high() {
    let mut map = IndexMap::new();
    map.insert(100, 10);
    let _ = map.remove_entry(&100);
}

#[test]
fn test_remove_entry_edge_case_low() {
    let mut map = IndexMap::new();
    map.insert(0, 10);
    let _ = map.remove_entry(&0);
}

#[test]
fn test_remove_entry_adjacent_keys() {
    let mut map = IndexMap::new();
    map.insert(50, 5);
    map.insert(51, 15);
    let _ = map.remove_entry(&51);
} 

#[test]
fn test_remove_entry_insertion_and_removal() {
    let mut map = IndexMap::new();
    map.insert(1, 1);
    map.insert(2, 2);
    let _ = map.remove_entry(&1);
    let _ = map.remove_entry(&2);
}

#[test]
fn test_remove_entry_multiple_removals() {
    let mut map = IndexMap::new();
    for i in 0..5 {
        map.insert(i, i * 10);
    }
    let _ = map.remove_entry(&2);
    let _ = map.remove_entry(&3);
    let _ = map.remove_entry(&1);
}

