// Answer 0

#[test]
fn test_allocate_initializes_map() {
    // Arrange: Create an instance of GrowingHashmapChar with the value type as i32
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    // Act: Call the allocate method
    hashmap.allocate();

    // Assert: Verify that the mask is correctly set and the map is initialized
    assert_eq!(hashmap.mask, 7); // 8 - 1 = 7
    assert!(hashmap.map.is_some());
    let map = hashmap.map.as_ref().unwrap();
    assert_eq!(map.len(), 8); // The length of the map should be 8
    assert!(map.iter().all(|elem| elem.key == 0 && elem.value == 0)); // All elements should be default
}

#[test]
fn test_allocate_called_multiple_times() {
    // Arrange: Create an instance of GrowingHashmapChar with the value type as u8
    let mut hashmap: GrowingHashmapChar<u8> = GrowingHashmapChar {
        used: 2,
        fill: 2,
        mask: 0,
        map: None,
    };

    // Act: Call the allocate method multiple times
    hashmap.allocate();
    let initial_mask = hashmap.mask;
    let initial_map_len = hashmap.map.as_ref().unwrap().len();

    // Call allocate again
    hashmap.allocate();

    // Assert: Check that the mask remains the same and map size is unchanged
    assert_eq!(hashmap.mask, initial_mask);
    assert_eq!(hashmap.map.as_ref().unwrap().len(), initial_map_len);
}

