// Answer 0

#[test]
fn test_shrink_to_within_capacity() {
    let mut map = IndexMapCore::<i32, i32>::with_capacity(10);
    for i in 0..5 {
        map.push_entry(0, i, i * 2);
    }
    map.shrink_to(3);
    assert_eq!(map.len(), 3);
    assert!(map.capacity() >= 3);
}

#[test]
fn test_shrink_to_exact_capacity() {
    let mut map = IndexMapCore::<i32, i32>::with_capacity(10);
    for i in 0..5 {
        map.push_entry(0, i, i * 2);
    }
    map.shrink_to(5);
    assert_eq!(map.len(), 5);
    assert!(map.capacity() >= 5);
}

#[test]
fn test_shrink_to_zero_capacity() {
    let mut map = IndexMapCore::<i32, i32>::with_capacity(10);
    for i in 0..5 {
        map.push_entry(0, i, i * 2);
    }
    map.shrink_to(0);
    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 5 but the index is 6. Expected index <= len")]
fn test_shrink_to_exceeding_current_length() {
    let mut map = IndexMapCore::<i32, i32>::new();
    for i in 0..5 {
        map.push_entry(0, i, i * 2);
    }
    // This should panic because we're trying to shrink to a value greater than current length
    map.shrink_to(6);
} 

#[test]
fn test_shrink_to_negative() {
    let mut map = IndexMapCore::<i32, i32>::with_capacity(10);
    for i in 0..5 {
        map.push_entry(0, i, i * 2);
    }
    // Shrinking to zero without panic
    map.shrink_to(0);
    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
}

