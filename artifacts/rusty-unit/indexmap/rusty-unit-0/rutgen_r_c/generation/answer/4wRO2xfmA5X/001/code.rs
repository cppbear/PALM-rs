// Answer 0

#[test]
fn test_swap_indices_valid() {
    let mut map = IndexMap::<&str, i32, RandomState>::new();
    map.insert("first", 1);
    map.insert("second", 2);
    map.insert("third", 3);
  
    // Swap valid indices
    map.swap_indices(0, 2);
  
    assert_eq!(map.get_index(0).unwrap().0, "third");
    assert_eq!(map.get_index(2).unwrap().0, "first");
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_swap_indices_out_of_bounds_high() {
    let mut map = IndexMap::<&str, i32, RandomState>::new();
    map.insert("first", 1);
    map.insert("second", 2);
  
    // Attempt to swap indices that are out of bounds
    map.swap_indices(0, 2);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_swap_indices_out_of_bounds_low() {
    let mut map = IndexMap::<&str, i32, RandomState>::new();
    map.insert("first", 1);
    map.insert("second", 2);
  
    // Attempt to swap indices that are out of bounds
    map.swap_indices(2, 1);
}

#[test]
fn test_swap_indices_same_index() {
    let mut map = IndexMap::<&str, i32, RandomState>::new();
    map.insert("first", 1);
    map.insert("second", 2);
  
    // Swap the same index (should not modify the map)
    map.swap_indices(0, 0);
  
    assert_eq!(map.get_index(0).unwrap().0, "first");
}

#[test]
fn test_swap_indices_two_elements() {
    let mut map = IndexMap::<&str, i32, RandomState>::new();
    map.insert("first", 1);
    map.insert("second", 2);
  
    // Swap both elements
    map.swap_indices(0, 1);
  
    assert_eq!(map.get_index(0).unwrap().0, "second");
    assert_eq!(map.get_index(1).unwrap().0, "first");
}

