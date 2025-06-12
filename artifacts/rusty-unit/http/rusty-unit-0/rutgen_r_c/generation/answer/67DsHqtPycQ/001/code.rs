// Answer 0

#[test]
fn test_iter_mut_empty() {
    let mut map: HeaderMap<String> = HeaderMap::with_capacity(10);
    let mut iter = map.iter_mut();
    
    assert_eq!(iter.entry, 0);
    assert_eq!(iter.cursor, None);
}

#[test]
fn test_iter_mut_single_entry() {
    let mut map: HeaderMap<String> = HeaderMap::with_capacity(10);
    map.insert("Key1".into(), "Value1".to_string());
    
    let mut iter = map.iter_mut();
    assert_eq!(iter.entry, 0);
    assert_eq!(iter.cursor, Some(Cursor::Head));

    if let Some((key, value)) = iter.next() {
        assert_eq!(key, "Key1");
        value.push_str("-boop");
        assert_eq!(value, "Value1-boop");
    } else {
        panic!("Expected an entry but found none.");
    }
}

#[test]
fn test_iter_mut_multiple_entries() {
    let mut map: HeaderMap<String> = HeaderMap::with_capacity(10);
    map.insert("Key1".into(), "Value1".to_string());
    map.append("Key1".into(), "Value2".to_string());
    
    let mut iter = map.iter_mut();
    assert_eq!(iter.entry, 0);
    assert_eq!(iter.cursor, Some(Cursor::Head));

    if let Some((key, value)) = iter.next() {
        assert_eq!(key, "Key1");
        value.push_str("-boop");
        assert_eq!(value, "Value1-boop");
    } 
    
    if let Some((key, value)) = iter.next() {
        assert_eq!(key, "Key1");
        value.push_str("-boop");
        assert_eq!(value, "Value2-boop");
    } else {
        panic!("Expected a second entry but found none.");
    }
}

#[test]
fn test_iter_mut_values_mutate_correctly() {
    let mut map: HeaderMap<String> = HeaderMap::with_capacity(10);
    map.insert("Key1".into(), "Value1".to_string());
    map.append("Key2".into(), "Value2".to_string());
    
    let mut iter = map.iter_mut();

    while let Some((key, value)) = iter.next() {
        value.push_str("-modified");
    }

    assert_eq!(map.get("Key1").unwrap(), "Value1-modified");
    assert_eq!(map.get("Key2").unwrap(), "Value2-modified");
}

