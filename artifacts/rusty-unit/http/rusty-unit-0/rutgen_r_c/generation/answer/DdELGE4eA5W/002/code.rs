// Answer 0

#[test]
fn test_value_iter_valid_index() {
    struct DummyValue;
    struct DummyHeaderMap {
        entries: Vec<Bucket<DummyValue>>,
    }

    impl HeaderMap<DummyValue> {
        pub fn new(entries: Vec<Bucket<DummyValue>>) -> Self {
            HeaderMap {
                mask: 0,
                indices: Box::new([]),
                entries,
                extra_values: vec![],
                danger: Danger::Green,
            }
        }
    }

    let entries = vec![
        Bucket {
            hash: 0,
            key: HeaderName::from_static("first"),
            value: DummyValue,
            links: None,
        },
        Bucket {
            hash: 1,
            key: HeaderName::from_static("second"),
            value: DummyValue,
            links: Some(Links { next: 1, tail: 2 }),
        },
    ];

    let header_map = DummyHeaderMap::new(entries);
    let iter = header_map.value_iter(Some(1));

    assert_eq!(iter.index, 1);
    assert_eq!(iter.front, Some(Cursor::Head));
    assert!(iter.back.is_some());
}

#[test]
#[should_panic]
fn test_value_iter_out_of_bounds_index() {
    struct DummyValue;
    struct DummyHeaderMap {
        entries: Vec<Bucket<DummyValue>>,
    }

    impl HeaderMap<DummyValue> {
        pub fn new(entries: Vec<Bucket<DummyValue>>) -> Self {
            HeaderMap {
                mask: 0,
                indices: Box::new([]),
                entries,
                extra_values: vec![],
                danger: Danger::Green,
            }
        }
    }

    let entries = vec![
        Bucket {
            hash: 0,
            key: HeaderName::from_static("first"),
            value: DummyValue,
            links: None,
        },
    ];

    let header_map = DummyHeaderMap::new(entries);
    // This index is out of bounds; it should panic.
    let _iter = header_map.value_iter(Some(1));
}

#[test]
fn test_value_iter_none_index() {
    struct DummyValue;
    struct DummyHeaderMap {
        entries: Vec<Bucket<DummyValue>>,
    }

    impl HeaderMap<DummyValue> {
        pub fn new(entries: Vec<Bucket<DummyValue>>) -> Self {
            HeaderMap {
                mask: 0,
                indices: Box::new([]),
                entries,
                extra_values: vec![],
                danger: Danger::Green,
            }
        }
    }

    let entries = vec![
        Bucket {
            hash: 0,
            key: HeaderName::from_static("first"),
            value: DummyValue,
            links: None,
        },
    ];

    let header_map = DummyHeaderMap::new(entries);
    let iter = header_map.value_iter(None);

    assert_eq!(iter.index, usize::MAX);
    assert!(iter.front.is_none());
    assert!(iter.back.is_none());
}

