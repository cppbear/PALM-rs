// Answer 0

#[test]
fn test_append_single_value() {
    // Setup
    let mut map = HeaderMap {
        mask: 0,
        indices: Box::new([]),
        entries: vec![
            Bucket {
                hash: 0,
                key: HeaderName::from("host").unwrap(),
                value: HeaderValue::from("world"),
                links: None,
            },
        ],
        extra_values: vec![],
        danger: Danger::new(),
    };
    
    // Act
    let mut entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 0,
    };
    entry.append(HeaderValue::from("earth"));

    // Assert
    let values = &map.extra_values;
    assert_eq!(values.len(), 1);
    assert_eq!(values[0].value, HeaderValue::from("earth"));
}

#[test]
fn test_append_multiple_values() {
    // Setup
    let mut map = HeaderMap {
        mask: 0,
        indices: Box::new([]),
        entries: vec![
            Bucket {
                hash: 0,
                key: HeaderName::from("host").unwrap(),
                value: HeaderValue::from("world"),
                links: None,
            },
        ],
        extra_values: vec![],
        danger: Danger::new(),
    };
    
    // Act
    let mut entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 0,
    };
    entry.append(HeaderValue::from("earth"));
    entry.append(HeaderValue::from("mars"));

    // Assert
    let values = &map.extra_values;
    assert_eq!(values.len(), 2);
    assert_eq!(values[0].value, HeaderValue::from("earth"));
    assert_eq!(values[1].value, HeaderValue::from("mars"));
} 

#[test]
fn test_append_empty_entry() {
    // Setup
    let mut map = HeaderMap {
        mask: 0,
        indices: Box::new([]),
        entries: vec![
            Bucket {
                hash: 0,
                key: HeaderName::from("host").unwrap(),
                value: HeaderValue::from("world"),
                links: None,
            },
        ],
        extra_values: vec![],
        danger: Danger::new(),
    };
    
    // Act
    let mut entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 0,
    };
    entry.append(HeaderValue::from("venus"));

    // Assert
    let values = &map.extra_values;
    assert_eq!(values.len(), 1);
    assert_eq!(values[0].value, HeaderValue::from("venus"));
}

