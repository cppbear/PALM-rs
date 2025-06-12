// Answer 0

#[test]
fn test_reverse_empty_map() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.reverse();
    assert_eq!(map.len(), 0);
}

#[test]
fn test_reverse_single_element_map() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(1, "one".to_string());
    map.reverse();
    assert_eq!(map.first(), Some((&1, &"one".to_string())));
}

#[test]
fn test_reverse_multiple_elements_map() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());
    map.reverse();
    
    let expected = vec![(&3, &"three".to_string()), (&2, &"two".to_string()), (&1, &"one".to_string())];
    let mut iter = map.iter();
    for (expected_k, expected_v) in expected {
        assert_eq!(iter.next(), Some((expected_k, expected_v)));
    }
}

#[test]
fn test_reverse_repeated_elements_map() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(1, "duplicate".to_string());
    map.insert(2, "duplicate".to_string());
    map.insert(3, "duplicate".to_string());
    map.reverse();
    
    let expected = vec![(&3, &"duplicate".to_string()), (&2, &"duplicate".to_string()), (&1, &"duplicate".to_string())];
    let mut iter = map.iter();
    for (expected_k, expected_v) in expected {
        assert_eq!(iter.next(), Some((expected_k, expected_v)));
    }
}

