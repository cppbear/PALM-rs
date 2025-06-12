// Answer 0

#[test]
fn test_iter_mut_with_non_empty_map() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(5, DefaultHashBuilder::new(), Global);
    map.insert("key1", 1);
    map.insert("key2", 2);
    map.insert("key3", 3);
    
    let mut iter = map.iter_mut();
    while let Some((key, val)) = iter.next() {
        *val *= 2; // Mutate values
    }
}

#[test]
fn test_iter_mut_with_multiple_entries() {
    let mut map: HashMap<usize, i32> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::new(), Global);
    for i in 1..=5 {
        map.insert(i, i as i32);
    }
    
    let mut iter = map.iter_mut();
    while let Some((_, val)) = iter.next() {
        *val += 10; // Mutate values
    }
}

#[test]
fn test_iter_mut_with_single_entry() {
    let mut map: HashMap<char, i32> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    map.insert('a', 5);
    
    let mut iter = map.iter_mut();
    if let Some((_, val)) = iter.next() {
        *val += 1; // Mutate value
    }
}

#[test]
fn test_iter_mut_with_capacity_zero() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    // Map starts empty, we expect no panic during iteration.
    let iter = map.iter_mut();
    assert!(iter.is_empty());
}

#[test]
fn test_iter_mut_large_capacity() {
    let mut map: HashMap<u32, u32> = HashMap::with_capacity_and_hasher_in(1_000_000, DefaultHashBuilder::new(), Global);
    for i in 1..=1_000_000 {
        map.insert(i, i);
    }
    
    let mut iter = map.iter_mut();
    while let Some((_, val)) = iter.next() {
        *val *= 2; // Mutate values
    }
}

#[test]
#[should_panic]
fn test_iter_mut_invalid_operation() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), Global);
    map.insert("key1", 1);
    map.insert("key2", 2);

    {
        let mut iter = map.iter_mut();
        if let Some((_, val)) = iter.next() {
            *val = 10; // Mutate value while holding a reference
        }
    }

    // This should not panic as the mutable borrow scope ends here.
    let _ = map.iter_mut().next();
}

