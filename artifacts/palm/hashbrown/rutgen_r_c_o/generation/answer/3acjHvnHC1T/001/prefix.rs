// Answer 0

#[test]
fn test_values_mut_empty_map() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    let mut values_iter = map.values_mut();
    // Perform operations while iterating
}

#[test]
fn test_values_mut_single_entry() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    map.insert("a", 1);
    let mut values_iter = map.values_mut();
    if let Some(val) = values_iter.next() {
        *val += 10; // mutate the value
    }
}

#[test]
fn test_values_mut_multiple_entries() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), Global);
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    let mut values_iter = map.values_mut();
    while let Some(val) = values_iter.next() {
        *val += 10; // mutate the values
    }
}

#[test]
#[should_panic]
fn test_values_mut_capacity_limit() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::new(), Global);
    for i in 0..10 {
        map.insert(&i.to_string(), i);
    }
    let mut values_iter = map.values_mut(); // Should be fine
    if values_iter.next().is_some() {
        // Attempt to mutate or iterate beyond capacity could be tested here
    }

    // Attempt to fill up to capacity again to trigger panic
    for i in 10..20 {
        map.insert(&i.to_string(), i); // should panic due to capacity being full
    }
}

