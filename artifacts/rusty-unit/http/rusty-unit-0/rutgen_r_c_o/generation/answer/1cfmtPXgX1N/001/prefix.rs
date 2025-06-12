// Answer 0

#[test]
fn test_into_mut_with_valid_entry() {
    let mut map = HeaderMap::default();
    map.insert(HeaderName::from_static("host"), "hello.world".to_string());
    map.append(HeaderName::from_static("host"), "hello.earth".to_string());
    
    if let OccupiedEntry { map: _, probe: _, index } = map.entry(HeaderName::from_static("host")).unwrap() {
        let value_ref = into_mut(OccupiedEntry { map: &mut map, probe: 0, index });
        value_ref.push_str("-test");
    }
}

#[test]
#[should_panic]
fn test_into_mut_with_empty_entry() {
    let mut map = HeaderMap::default();
    map.insert(HeaderName::from_static("host"), "host_value".to_string());
    
    if let OccupiedEntry { map: _, probe: _, index } = map.entry(HeaderName::from_static("nonexistent")).unwrap() {
        let _value_ref = into_mut(OccupiedEntry { map: &mut map, probe: 0, index });
    }
}

#[test]
fn test_into_mut_multiple_insertions() {
    let mut map = HeaderMap::default();
    map.insert(HeaderName::from_static("accept"), "text/html".to_string());
    map.append(HeaderName::from_static("accept"), "application/json".to_string());
    
    if let OccupiedEntry { map: _, probe: _, index } = map.entry(HeaderName::from_static("accept")).unwrap() {
        let value_ref = into_mut(OccupiedEntry { map: &mut map, probe: 0, index });
        value_ref.push_str("; application/xml");
    }
}

#[test]
fn test_into_mut_with_edge_index() {
    let mut map = HeaderMap::default();
    let max_entries = MAX_SIZE as usize;
    
    for i in 0..max_entries {
        map.insert(HeaderName::from_static(&format!("key{}", i)), format!("value{}", i));
    }
    
    let last_index = max_entries - 1;
    if let OccupiedEntry { map: _, probe: _, index } = map.entry(HeaderName::from_static(&format!("key{}", last_index))).unwrap() {
        let value_ref = into_mut(OccupiedEntry { map: &mut map, probe: 0, index });
        value_ref.push_str("-last");
    }
}

#[test]
fn test_into_mut_with_mid_index() {
    let mut map = HeaderMap::default();
    map.insert(HeaderName::from_static("test-key"), "initial-value".to_string());
    
    if let OccupiedEntry { map: _, probe: _, index } = map.entry(HeaderName::from_static("test-key")).unwrap() {
        let value_ref = into_mut(OccupiedEntry { map: &mut map, probe: 0, index });
        *value_ref = "updated-value".to_string();
    }
}

