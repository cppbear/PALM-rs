// Answer 0

#[test]
fn test_value_iter_with_valid_index() {
    #[derive(Clone)]
    struct DummyValue;
    let mut header_map = HeaderMap::<DummyValue>::with_capacity(10);
    header_map.entries.push(Bucket {
        hash: 0,
        key: HeaderName::from_static("key1"),
        value: DummyValue,
        links: None,
    });
    header_map.entries.push(Bucket {
        hash: 1,
        key: HeaderName::from_static("key2"),
        value: DummyValue,
        links: Some(Links { next: 1, tail: 0 }),
    });

    let iter = header_map.value_iter(Some(1));
    assert_eq!(iter.index, 1);
    assert!(matches!(iter.front, Some(Cursor::Head)));
    assert!(matches!(iter.back, Some(Cursor::Values(1))));
}

#[test]
fn test_value_iter_with_none_index() {
    #[derive(Clone)]
    struct DummyValue;
    let header_map = HeaderMap::<DummyValue>::with_capacity(10);

    let iter = header_map.value_iter(None);
    assert_eq!(iter.index, usize::MAX);
    assert!(iter.front.is_none());
    assert!(iter.back.is_none());
}

