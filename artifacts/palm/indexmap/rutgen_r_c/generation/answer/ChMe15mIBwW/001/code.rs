// Answer 0

#[test]
fn test_move_index_valid() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    for i in 0..5 {
        map.entries.push(Bucket { hash: HashValue::default(), key: i, value: i });
    }
    // Move index from 0 to 2
    let from = 0;
    let to = 2;
    map.move_index(from, to);
    assert_eq!(map.entries[0].key, 1);
    assert_eq!(map.entries[1].key, 2);
    assert_eq!(map.entries[2].key, 0);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_move_index_out_of_bounds() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    for i in 0..5 {
        map.entries.push(Bucket { hash: HashValue::default(), key: i, value: i });
    }
    // Trying to move index that is out of bounds
    let from = 5;  // Out of bounds
    let to = 2;
    map.move_index(from, to);
}

#[test]
fn test_move_index_to_same_position() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    for i in 0..5 {
        map.entries.push(Bucket { hash: HashValue::default(), key: i, value: i });
    }
    // Move index from 1 to 1 (same position)
    let from = 1;
    let to = 1;
    map.move_index(from, to);
    // Ensure no change in order
    assert_eq!(map.entries[1].key, 1);
}

#[test]
fn test_move_index_reverse_order() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    for i in 0..5 {
        map.entries.push(Bucket { hash: HashValue::default(), key: i, value: i });
    }
    // Move index from 3 to 1 (reversing order)
    let from = 3;
    let to = 1;
    map.move_index(from, to);
    assert_eq!(map.entries[1].key, 3);
    assert_eq!(map.entries[2].key, 1);
}

