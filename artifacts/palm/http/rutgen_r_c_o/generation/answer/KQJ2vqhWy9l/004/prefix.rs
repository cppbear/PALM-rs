// Answer 0

#[test]
fn test_rebuild_with_valid_entries() {
    let mut header_map: HeaderMap<i32> = HeaderMap::with_capacity(16);
    header_map.entries.push(Bucket {
        hash: HashValue(1),
        key: HeaderName { inner: Repr::default() }, // Assuming Repr has a default
        value: 42,
        links: None,
    });
    header_map.entries.push(Bucket {
        hash: HashValue(2),
        key: HeaderName { inner: Repr::default() },
        value: 43,
        links: None,
    });
    // Populate indices to ensure they are occupied
    header_map.indices = vec![Pos::new(0, HashValue(1)), Pos::new(1, HashValue(2))].into_boxed_slice();
    header_map.danger = Danger::Green;
    
    header_map.rebuild();
}

#[test]
fn test_rebuild_with_empty_entries() {
    let mut header_map: HeaderMap<i32> = HeaderMap::with_capacity(16);
    header_map.rebuild(); // Testing behavior with no entries
}

#[test]
fn test_rebuild_with_full_capacity() {
    let mut header_map: HeaderMap<i32> = HeaderMap::with_capacity(16);
    for i in 0..16 {
        header_map.entries.push(Bucket {
            hash: HashValue(i as u16),
            key: HeaderName { inner: Repr::default() },
            value: i as i32,
            links: None,
        });
    }
    // Fill indices to ensure certain states
    header_map.indices = (0..16).map(|i| Pos::new(i, HashValue(i as u16))).collect::<Vec<_>>().into_boxed_slice();
    header_map.danger = Danger::Green;

    header_map.rebuild();
}

#[test]
fn test_rebuild_with_displaced_entries() {
    let mut header_map: HeaderMap<i32> = HeaderMap::with_capacity(16);
    header_map.indices = vec![Pos::new(0, HashValue(1)), Pos::new(1, HashValue(2))].into_boxed_slice();
    header_map.entries.push(Bucket {
        hash: HashValue(3),
        key: HeaderName { inner: Repr::default() },
        value: 44,
        links: None,
    });
    header_map.entries.push(Bucket {
        hash: HashValue(1),
        key: HeaderName { inner: Repr::default() },
        value: 45,
        links: None,
    });
    header_map.danger = Danger::Green;

    header_map.rebuild();
}

