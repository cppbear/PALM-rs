// Answer 0

#[test]
fn test_growing_hashmap_char_get_mut_initial_allocation() {
    struct TestValue(u32);
    
    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    let value_ref: &mut TestValue = hashmap.get_mut(1);
    value_ref.0 = 42;

    assert_eq!(value_ref.0, 42);
    assert_eq!(hashmap.used, 1);
    assert_eq!(hashmap.fill, 1);
}

#[test]
fn test_growing_hashmap_char_get_mut_fill_constraint() {
    struct TestValue(u32);
    
    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 7,
        map: Some(vec![GrowingHashmapMapElemChar::default(); 8]),
    };

    // Manually set the map to a value that satisfies the conditions
    hashmap.fill = 5; // Fill should now be 5
    hashmap.used = 5; // Assume we have previously filled 5 slots

    let value_ref: &mut TestValue = hashmap.get_mut(2);
    value_ref.0 = 50;

    assert_eq!(value_ref.0, 50);
    assert_eq!(hashmap.used, 6);
    assert_eq!(hashmap.fill, 6);
}

#[test]
#[should_panic]
fn test_growing_hashmap_char_get_mut_panic_on_unallocated_map() {
    let mut hashmap: GrowingHashmapChar<u32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    let _value_ref: &mut u32 = hashmap.get_mut(3); // This should trigger an allocation
}

#[test]
#[should_panic]
fn test_growing_hashmap_char_get_mut_panic_on_accessing_empty_slot() {
    struct TestValue(u32);
    
    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 7,
        map: Some(vec![GrowingHashmapMapElemChar::default(); 8]),
    };

    let _value_ref: &mut TestValue = hashmap.get_mut(1);
    
    // Fill the map to avoid panic on empty slot
    hashmap.fill = 5; 
    hashmap.used = 5;

    // Accessing a key that isn't present will now panic when attempting to access the empty slot
    let _value_ref2: &mut TestValue = hashmap.get_mut(6);
}

