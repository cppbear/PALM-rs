// Answer 0

#[test]
fn get_mut_test_valid_entry() {
    let mut map = HeaderMap {
        mask: 0,
        indices: Box::new([]),
        entries: vec![Bucket {
            hash: 0,
            key: HeaderName::from("host".to_string()).unwrap(), // Assuming valid conversion
            value: "hello.world".to_string(),
            links: None,
        }],
        extra_values: vec![],
        danger: Danger::default(), // Assuming Danger has a default method
    };
    
    let mut entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 0,
    };
    
    let value_mut: &mut String = entry.get_mut();
    value_mut.push_str("-2");
    
    assert_eq!(entry.get(), &"hello.world-2".to_string());
}

#[test]
#[should_panic]
fn get_mut_test_no_values() {
    let mut map = HeaderMap {
        mask: 0,
        indices: Box::new([]),
        entries: vec![], // No entries here
        extra_values: vec![],
        danger: Danger::default(),
    };
    
    let entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 0, // Invalid index since there are no entries
    };
    
    // This should panic because there are no values to get_mut
    entry.get_mut();
} 

#[test]
fn get_mut_test_multiple_values() {
    let mut map = HeaderMap {
        mask: 0,
        indices: Box::new([]),
        entries: vec![Bucket {
            hash: 0,
            key: HeaderName::from("host".to_string()).unwrap(),
            value: vec!["hello.world".to_string(), "another.value".to_string()],
            links: None,
        }],
        extra_values: vec![],
        danger: Danger::default(),
    };
    
    let mut entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 0,
    };
    
    let value_mut: &mut Vec<String> = entry.get_mut();
    value_mut.push("new.value".to_string());
    
    assert_eq!(entry.get(), &vec!["hello.world", "another.value", "new.value"]);
}

