// Answer 0

#[test]
fn test_value_iter() {
    use crate::header::{HeaderMap, HeaderValue};
    
    let mut map = HeaderMap::with_capacity(2);
    let key = "host".parse::<HeaderName>().unwrap();
    map.insert(key.clone(), HeaderValue::from("world")).unwrap();
    map.append(key.clone(), HeaderValue::from("earth"));
    
    let occupied_entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 0,
    };
    
    let mut iter = occupied_entry.iter();
    assert_eq!(iter.next().unwrap(), &HeaderValue::from("world"));
    assert_eq!(iter.next().unwrap(), &HeaderValue::from("earth"));
    assert!(iter.next().is_none());
}

#[test]
fn test_value_iter_empty() {
    use crate::header::{HeaderMap, HeaderValue};

    let mut map = HeaderMap::new();
    let key = "host".parse::<HeaderName>().unwrap();
    
    if let Entry::Occupied(_) = map.entry(key) {
        let occupied_entry = OccupiedEntry {
            map: &mut map,
            probe: 0,
            index: 0,
        };
        let mut iter = occupied_entry.iter();
        assert!(iter.next().is_none());
    }
}

