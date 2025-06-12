// Answer 0

#[test]
fn test_values_mut() {
    // Create a simple HashMap with i32 keys and values
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::default(), Global);
    
    // Insert values into the map
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    // Modify the values using values_mut
    for val in map.values_mut() {
        *val += 10;
    }
    
    // Check the length remains the same
    assert_eq!(map.len(), 3);
    
    // Collect the values into a vector for validation
    let mut vec: Vec<i32> = Vec::new();
    
    for val in map.values() {
        vec.push(*val);
    }
    
    // The modified values should be [20, 30, 40]
    vec.sort_unstable();
    assert_eq!(vec, [20, 30, 40]);
}

#[test]
fn test_values_mut_empty() {
    // Create an empty HashMap
    let mut map: HashMap<i32, i32> = HashMap::new();
    
    // Since the map is empty, values_mut should yield no values
    let mut vec: Vec<i32> = Vec::new();
    
    for val in map.values_mut() {
        vec.push(*val); // This should not happen
    }
    
    // The length of the vector should remain 0
    assert_eq!(vec.len(), 0);
}

#[test]
fn test_values_mut_single() {
    // Create a HashMap with one entry
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::default(), Global);
    map.insert(1, 10);
    
    // Modify the single value using values_mut
    for val in map.values_mut() {
        *val *= 2;
    }
    
    // Verify the modification
    assert_eq!(map.get(&1), Some(&20));
}

