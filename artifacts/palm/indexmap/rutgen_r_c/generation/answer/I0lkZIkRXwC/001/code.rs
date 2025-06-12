// Answer 0

#[test]
fn test_pop_empty_index_map() {
    use crate::IndexMap;

    let mut map: IndexMap<i32, i32, ()> = IndexMap::new();
    assert_eq!(map.pop(), None);
}

#[test]
fn test_pop_single_entry_index_map() {
    use crate::IndexMap;

    let mut map: IndexMap<i32, i32, ()> = IndexMap::new();
    map.insert(1, 100); // Assuming insert method is available
    assert_eq!(map.pop(), Some((1, 100)));
    assert_eq!(map.pop(), None);
}

#[test]
fn test_pop_multiple_entries_index_map() {
    use crate::IndexMap;

    let mut map: IndexMap<i32, i32, ()> = IndexMap::new();
    map.insert(1, 100); // Assuming insert method is available
    map.insert(2, 200);
    map.insert(3, 300);
    
    assert_eq!(map.pop(), Some((3, 300)));  // Last entry popped
    assert_eq!(map.pop(), Some((2, 200)));  // Second last entry popped
    assert_eq!(map.pop(), Some((1, 100)));  // Last remaining entry popped
    assert_eq!(map.pop(), None);             // No entries left
} 

#[test]
fn test_pop_with_order_preservation() {
    use crate::IndexMap;

    let mut map: IndexMap<i32, i32, ()> = IndexMap::new();
    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(3, 300);

    // Perform multiple pops
    assert_eq!(map.pop(), Some((3, 300)));
    assert_eq!(map.pop(), Some((2, 200)));
    assert_eq!(map.pop(), Some((1, 100)));
}

