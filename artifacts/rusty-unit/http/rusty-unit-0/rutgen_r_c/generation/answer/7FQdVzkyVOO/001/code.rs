// Answer 0

#[test]
fn test_value_iter_mut_empty() {
    let mut map: HeaderMap<String> = HeaderMap::with_capacity(1);
    let entry = map.entry("host");
    if let Entry::Vacant(_) = entry {
        // No panic expected, simply validating the state
    } else {
        panic!("Expected entry to be vacant");
    }
}

#[test]
fn test_value_iter_mut_single_value() {
    let mut map = HeaderMap::with_capacity(2);
    map.insert("host", "world".to_string());
    
    if let Entry::Occupied(mut entry) = map.entry("host") {
        let mut iter = entry.iter_mut();
        let value = iter.next().unwrap();
        assert_eq!(value, "world");
        value.push_str("-boop");
        assert_eq!(value, "world-boop");
    } else {
        panic!("Expected entry to be occupied");
    }
}

#[test]
fn test_value_iter_mut_multiple_values() {
    let mut map = HeaderMap::with_capacity(3);
    map.insert("host", "world".to_string());
    map.append("host", "earth".to_string());
    
    if let Entry::Occupied(mut entry) = map.entry("host") {
        let mut iter = entry.iter_mut();
        let first_value = iter.next().unwrap();
        assert_eq!(first_value, "world");
        first_value.push_str("-boop");
        
        let second_value = iter.next().unwrap();
        assert_eq!(second_value, "earth");
        second_value.push_str("-boop");
        
        assert_eq!(first_value, "world-boop");
        assert_eq!(second_value, "earth-boop");
    } else {
        panic!("Expected entry to be occupied");
    }
}

