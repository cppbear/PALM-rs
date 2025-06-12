// Answer 0

#[test]
fn test_into_mut_panics_when_no_values() {
    let mut map = HeaderMap::<String> {
        mask: 0,
        indices: Box::new([]),
        entries: Vec::new(),
        extra_values: Vec::new(),
        danger: Danger::default(),
    };
    
    let entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 0,
    };
    
    let result = std::panic::catch_unwind(|| {
        entry.into_mut()
    });

    assert!(result.is_err());
}

#[test]
fn test_into_mut_success() {
    let key = HeaderName::try_from("host").unwrap();
    
    let mut map = HeaderMap::<String> {
        mask: 0,
        indices: Box::new([Pos::Vacant]),
        entries: vec![Bucket {
            hash: HashValue::default(),
            key: key.clone(),
            value: "hello.world".to_string(),
            links: None,
        }],
        extra_values: Vec::new(),
        danger: Danger::default(),
    };

    let entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 0,
    };

    let value_ref = entry.into_mut();
    value_ref.push_str("-2");

    assert_eq!(map.entries[0].value, "hello.world-2");
}

