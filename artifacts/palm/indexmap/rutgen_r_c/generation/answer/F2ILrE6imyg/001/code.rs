// Answer 0

#[test]
fn test_values_empty() {
    struct DummyHashBuilder;

    let map: IndexMap<i32, i32, DummyHashBuilder> = IndexMap::with_capacity_and_hasher(0, DummyHashBuilder);
    let mut values_iter = map.values();
    let values: Vec<_> = values_iter.iter.collect();
    
    assert_eq!(values.len(), 0);
}

#[test]
fn test_values_one_element() {
    struct DummyHashBuilder;

    let mut map: IndexMap<i32, i32, DummyHashBuilder> = IndexMap::with_capacity_and_hasher(1, DummyHashBuilder);
    map.insert(1, 100);
    
    let mut values_iter = map.values();
    let values: Vec<_> = values_iter.iter.collect();
    
    assert_eq!(values.len(), 1);
    assert_eq!(values[0], 100);
}

#[test]
fn test_values_multiple_elements() {
    struct DummyHashBuilder;

    let mut map: IndexMap<i32, i32, DummyHashBuilder> = IndexMap::with_capacity_and_hasher(3, DummyHashBuilder);
    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(3, 300);
    
    let mut values_iter = map.values();
    let values: Vec<_> = values_iter.iter.collect();
    
    assert_eq!(values.len(), 3);
    assert_eq!(values[0], 100);
    assert_eq!(values[1], 200);
    assert_eq!(values[2], 300);
}

#[test]
fn test_values_after_clear() {
    struct DummyHashBuilder;

    let mut map: IndexMap<i32, i32, DummyHashBuilder> = IndexMap::with_capacity_and_hasher(2, DummyHashBuilder);
    map.insert(1, 100);
    map.insert(2, 200);
    
    map.clear();
    
    let mut values_iter = map.values();
    let values: Vec<_> = values_iter.iter.collect();
    
    assert_eq!(values.len(), 0);
}

