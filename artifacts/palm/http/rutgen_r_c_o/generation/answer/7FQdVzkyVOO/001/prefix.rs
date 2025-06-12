// Answer 0

#[test]
fn test_iter_mut_empty() {
    let mut map = HeaderMap::with_capacity(16);
    let entry = map.entry("test_key");
    if let Entry::Vacant(_) = entry {}
}

#[test]
fn test_iter_mut_single_value() {
    let mut map = HeaderMap::with_capacity(16);
    map.insert("single_key", "single_value".to_string());
    if let Entry::Occupied(mut entry) = map.entry("single_key") {
        let mut iter = entry.iter_mut();
        let value = iter.next().unwrap();
        value.push_str("-modified");
    }
}

#[test]
fn test_iter_mut_multiple_values() {
    let mut map = HeaderMap::with_capacity(16);
    map.append("multi_key", "value_one".to_string());
    map.append("multi_key", "value_two".to_string());
    if let Entry::Occupied(mut entry) = map.entry("multi_key") {
        for value in entry.iter_mut() {
            value.push_str("-modified");
        }
    }
}

#[test]
fn test_iter_mut_capacity_limit() {
    let max_capacity = 32768;
    let mut map = HeaderMap::with_capacity(max_capacity);
    for i in 0..max_capacity {
        map.append(i.to_string().as_str(), "value".to_string());
    }
    if let Entry::Occupied(mut entry) = map.entry("0") {
        for value in entry.iter_mut() {
            value.push_str("-modified");
        }
    }
}

#[test]
fn test_iter_mut_keys_length() {
    let mut map = HeaderMap::with_capacity(16);
    map.append("key1", "value1".to_string());
    map.append("key1", "value2".to_string());
    
    if let Entry::Occupied(mut entry) = map.entry("key1") {
        assert_eq!(entry.iter_mut().count(), 2);
    }
}

#[test]
fn test_iter_mut_remove_entry() {
    let mut map = HeaderMap::with_capacity(16);
    map.append("removal_key", "value_one".to_string());
    map.append("removal_key", "value_two".to_string());
    
    if let Entry::Occupied(entry) = map.entry("removal_key") {
        let first_value = entry.iter_mut().next().unwrap();
        let removed_value = entry.remove();
        assert_eq!(removed_value, "value_one".to_string());
        first_value.push_str("-post_remove");
    }
}

