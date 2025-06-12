// Answer 0

#[test]
fn test_retain_empty_map() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    map.retain(|_k, _v| false);
    assert_eq!(map.len(), 0);
}

#[test]
fn test_retain_no_elements_meet_condition() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(4, DefaultHashBuilder::new(), Global);
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    assert_eq!(map.len(), 3);
    
    map.retain(|_k, _v| false); // No elements should be retained
    assert_eq!(map.len(), 0);
}

#[test]
fn test_retain_all_elements_meet_condition() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), Global);
    map.insert(2, 20);
    map.insert(4, 40);
    map.insert(6, 60);
    assert_eq!(map.len(), 3);
    
    map.retain(|_k, _v| true); // All elements should be retained
    assert_eq!(map.len(), 3);
}

#[test]
fn test_retain_some_elements_meet_condition() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(4, DefaultHashBuilder::new(), Global);
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    assert_eq!(map.len(), 3);
    
    map.retain(|k, _v| *k % 2 == 0); // Retain only even keys
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&2), Some(&20));
}

#[test]
fn test_retain_with_mutation() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), Global);
    map.insert(1, 10);
    map.insert(2, 20);
    assert_eq!(map.len(), 2);
    
    map.retain(|k, v| {
        if *k == 1 {
            *v += 10; // Mutate the value
            return true;
        }
        false
    });
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&1), Some(&20)); // The value should be updated
}

