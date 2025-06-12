// Answer 0

#[test]
fn test_lookup_existing_key() {
    struct TestValueType(i32);
    
    let mut hashmap = GrowingHashmapChar::<TestValueType> {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![GrowingHashmapMapElemChar { key: 1, value: TestValueType(42) }, GrowingHashmapMapElemChar::default()]),
    };
    
    let result = hashmap.lookup(1);
    assert_eq!(result, 0);
}

#[test]
fn test_lookup_nonexistent_key() {
    struct TestValueType(i32);
    
    let mut hashmap = GrowingHashmapChar::<TestValueType> {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![GrowingHashmapMapElemChar::default(), GrowingHashmapMapElemChar { key: 2, value: TestValueType(24) }]),
    };
    
    let result = hashmap.lookup(1); // Should not panic
    assert_eq!(result, 0);
}

#[test]
fn test_lookup_with_default_value() {
    struct TestValueType(i32);
    
    let mut hashmap = GrowingHashmapChar::<TestValueType> {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: TestValueType(42) },
            GrowingHashmapMapElemChar::default()]),
    };
    
    let result = hashmap.lookup(0); // Should not panic
    assert_eq!(result, 1);
}

