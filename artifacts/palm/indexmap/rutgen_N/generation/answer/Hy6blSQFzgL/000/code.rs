// Answer 0

#[test]
fn test_sorted_by_stable_sort() {
    use indexmap::IndexMap;
    use std::cmp::Ordering;

    // Create an instance of IndexMap and initialize it with test data
    let mut map = IndexMap::new();
    map.insert("a", 2);
    map.insert("b", 1);
    map.insert("c", 3);
    
    // Define the comparison function to sort by value
    let cmp = |_, v1: &i32, _, v2: &i32| v1.cmp(v2);

    // Call the sorted_by method
    let sorted_iter = map.clone().sorted_by(cmp);
    
    // Collect the sorted results into a vector
    let sorted: Vec<_> = sorted_iter.collect();
    
    // Assert that the key-value pairs are in the expected sorted order
    assert_eq!(sorted, vec![("b", 1), ("a", 2), ("c", 3)]);
}

#[test]
fn test_sorted_by_no_elements() {
    use indexmap::IndexMap;
    use std::cmp::Ordering;

    // Create an empty IndexMap
    let map: IndexMap<&str, i32> = IndexMap::new();
    
    // Define the comparison function (irrelevant for empty map)
    let cmp = |_, _, _, _| Ordering::Equal;

    // Call the sorted_by method
    let sorted_iter = map.sorted_by(cmp);
    
    // Collect the sorted results into a vector
    let sorted: Vec<_> = sorted_iter.collect();
    
    // Assert that the result is an empty vector
    assert!(sorted.is_empty());
}

#[test]
fn test_sorted_by_with_equal_values() {
    use indexmap::IndexMap;
    use std::cmp::Ordering;

    // Create an IndexMap with equal values
    let mut map = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 1);
    map.insert("c", 1);
    
    // Define the comparison function to sort by key
    let cmp = |k1: &str, _, k2: &str, _| k1.cmp(k2);

    // Call the sorted_by method
    let sorted_iter = map.clone().sorted_by(cmp);
    
    // Collect the sorted results into a vector
    let sorted: Vec<_> = sorted_iter.collect();
    
    // Assert that the keys are in the expected sorted order
    assert_eq!(sorted, vec![("a", 1), ("b", 1), ("c", 1)]);
}

