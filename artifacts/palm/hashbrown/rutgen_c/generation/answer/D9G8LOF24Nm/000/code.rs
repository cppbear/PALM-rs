// Answer 0

#[test]
fn test_values_iterator() {
    // Define a simple HashMap for testing
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), Global);

    // Insert some key-value pairs
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    // Verify the length of the map
    assert_eq!(map.len(), 3);

    // Create a vector to collect values from the iterator
    let mut vec: Vec<i32> = Vec::new();

    // Iterate over values and collect them
    for val in map.values() {
        vec.push(*val);
    }

    // Sort the collected values
    vec.sort_unstable();

    // Check that the collected values match the expected output
    assert_eq!(vec, [1, 2, 3]);

    // Ensure the length of the map is still correct
    assert_eq!(map.len(), 3);
}

#[test]
fn test_values_empty_iterator() {
    // Define an empty HashMap
    let map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);

    // Create a vector to collect values from the iterator
    let vec: Vec<i32> = map.values().collect();

    // Check that the collected values are empty
    assert!(vec.is_empty());
}

#[test]
fn test_values_single_element() {
    // Define a HashMap and insert a single key-value pair
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    map.insert("a", 42);

    // Create a vector to collect the single value from the iterator
    let mut vec: Vec<i32> = Vec::new();
    for val in map.values() {
        vec.push(*val);
    }

    // Verify that the collected value is correct
    assert_eq!(vec, [42]);
}

