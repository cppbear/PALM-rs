// Answer 0

#[test]
fn test_pop_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    assert_eq!(map.pop(), None);
}

#[test]
fn test_pop_single_element_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    assert_eq!(map.pop(), Some((1, 10)));
    assert_eq!(map.pop(), None);
}

#[test]
fn test_pop_multiple_elements_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    assert_eq!(map.pop(), Some((3, 30)));
    assert_eq!(map.pop(), Some((2, 20)));
    assert_eq!(map.pop(), Some((1, 10)));
    assert_eq!(map.pop(), None);
}

#[test]
fn test_pop_order_preservation() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    assert_eq!(map.pop(), Some((3, 30)));
    assert_eq!(map.pop(), Some((2, 20)));
    assert_eq!(map.pop(), Some((1, 10)));
}

