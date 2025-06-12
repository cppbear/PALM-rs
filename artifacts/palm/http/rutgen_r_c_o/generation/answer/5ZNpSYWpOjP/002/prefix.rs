// Answer 0

#[test]
fn test_or_try_insert_occupied_entry() {
    let mut map: HeaderMap<u32> = HeaderMap::default();
    let key = HeaderName::try_from("test-header").unwrap();
    map.insert(key.clone(), 1);
    
    let entry = map.entry(key).or_try_insert(0).unwrap();
    *entry += 1;
}

#[test]
fn test_or_try_insert_occupied_entry_multiple() {
    let mut map: HeaderMap<u32> = HeaderMap::default();
    let key = HeaderName::try_from("header-1").unwrap();
    map.insert(key.clone(), 2);
    
    let entry = map.entry(key).or_try_insert(0).unwrap();
    *entry += 3;
}

#[test]
fn test_or_try_insert_occupied_entry_edge() {
    let mut map: HeaderMap<u32> = HeaderMap::default();
    let key = HeaderName::try_from("edge-header").unwrap();
    map.insert(key.clone(), u32::MAX);
    
    let entry = map.entry(key).or_try_insert(1).unwrap();
    *entry += 1; 
}

