// Answer 0

#[test]
fn test_rebuild_empty() {
    let mut map: HeaderMap<u32> = HeaderMap {
        mask: 0,
        indices: Box::new([]),
        entries: vec![],
        extra_values: vec![],
        danger: Danger::Green,
    };

    map.rebuild();
}

#[test]
fn test_rebuild_with_capacity() {
    let mut map: HeaderMap<u32> = HeaderMap {
        mask: 0,
        indices: Box::new([]),
        entries: vec![Bucket {
            hash: HashValue(0),
            key: HeaderName { inner: Repr::Custom },
            value: 42,
            links: None,
        }],
        extra_values: vec![],
        danger: Danger::Green,
    };

    map.rebuild();
}

#[test]
fn test_rebuild_large_capacity() {
    let mut map: HeaderMap<u32> = HeaderMap {
        mask: 0,
        indices: Box::new([]),
        entries: (0..MAX_SIZE as u32).map(|i| Bucket {
            hash: HashValue(i as u16),
            key: HeaderName { inner: Repr::Custom },
            value: i,
            links: None,
        }).collect(),
        extra_values: vec![],
        danger: Danger::Green,
    };

    map.rebuild();
}

#[test]
#[should_panic]
fn test_rebuild_no_indices() {
    let mut map: HeaderMap<u32> = HeaderMap {
        mask: 0,
        indices: Box::new([]),
        entries: vec![Bucket {
            hash: HashValue(0),
            key: HeaderName { inner: Repr::Custom },
            value: 42,
            links: None,
        }],
        extra_values: vec![],
        danger: Danger::Green,
    };

    map.rebuild();
}

