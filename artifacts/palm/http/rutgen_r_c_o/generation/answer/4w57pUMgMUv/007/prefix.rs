// Answer 0

#[test]
fn test_next_unsafe_valid_case() {
    let mut header_map = HeaderMap {
        mask: 15,
        indices: Box::from([Pos::new(0); 16]),
        entries: vec![
            Bucket {
                hash: HashValue::default(),
                key: HeaderName { inner: Repr::default() },
                value: HeaderValue::new("value1"),
                links: Some(Links { next: 1, tail: 1 }),
            },
            Bucket {
                hash: HashValue::default(),
                key: HeaderName { inner: Repr::default() },
                value: HeaderValue::new("value2"),
                links: None,
            },
        ],
        extra_values: vec![ExtraValue { value: HeaderValue::new("extra1"), prev: Link::Entry(0), next: Link::Extra(1) }],
        danger: Danger::default(),
    };

    let mut iter = IterMut {
        map: &mut header_map as *mut HeaderMap<HeaderValue>,
        entry: 0,
        cursor: Some(Cursor::Head),
        lt: PhantomData,
    };

    let result = iter.next_unsafe();
}

#[test]
fn test_next_unsafe_with_links() {
    let mut header_map = HeaderMap {
        mask: 15,
        indices: Box::from([Pos::new(0); 16]),
        entries: vec![
            Bucket {
                hash: HashValue::default(),
                key: HeaderName { inner: Repr::default() },
                value: HeaderValue::new("value1"),
                links: Some(Links { next: 1, tail: 1 }),
            },
            Bucket {
                hash: HashValue::default(),
                key: HeaderName { inner: Repr::default() },
                value: HeaderValue::new("value2"),
                links: Some(Links { next: 2, tail: 2 }),
            },
            Bucket {
                hash: HashValue::default(),
                key: HeaderName { inner: Repr::default() },
                value: HeaderValue::new("value3"),
                links: None,
            },
        ],
        extra_values: vec![
            ExtraValue { value: HeaderValue::new("extra1"), prev: Link::Entry(0), next: Link::Extra(1) },
            ExtraValue { value: HeaderValue::new("extra2"), prev: Link::Entry(1), next: Link::Entry(2) },
        ],
        danger: Danger::default(),
    };

    let mut iter = IterMut {
        map: &mut header_map as *mut HeaderMap<HeaderValue>,
        entry: 0,
        cursor: Some(Cursor::Head),
        lt: PhantomData,
    };

    let _result = iter.next_unsafe();
    let _result = iter.next_unsafe();
}

#[test]
fn test_next_unsafe_with_multiple_entries() {
    let mut header_map = HeaderMap {
        mask: 15,
        indices: Box::from([Pos::new(0); 16]),
        entries: vec![
            Bucket {
                hash: HashValue::default(),
                key: HeaderName { inner: Repr::default() },
                value: HeaderValue::new("value1"),
                links: Some(Links { next: 1, tail: 1 }),
            },
            Bucket {
                hash: HashValue::default(),
                key: HeaderName { inner: Repr::default() },
                value: HeaderValue::new("value2"),
                links: Some(Links { next: 2, tail: 2 }),
            },
            Bucket {
                hash: HashValue::default(),
                key: HeaderName { inner: Repr::default() },
                value: HeaderValue::new("value3"),
                links: Some(Links { next: 3, tail: 3 }),
            },
            Bucket {
                hash: HashValue::default(),
                key: HeaderName { inner: Repr::default() },
                value: HeaderValue::new("value4"),
                links: None,
            },
        ],
        extra_values: vec![
            ExtraValue { value: HeaderValue::new("extra1"), prev: Link::Entry(0), next: Link::Extra(1) },
            ExtraValue { value: HeaderValue::new("extra2"), prev: Link::Entry(1), next: Link::Extra(2) },
            ExtraValue { value: HeaderValue::new("extra3"), prev: Link::Entry(2), next: Link::Entry(3) },
        ],
        danger: Danger::default(),
    };

    let mut iter = IterMut {
        map: &mut header_map as *mut HeaderMap<HeaderValue>,
        entry: 0,
        cursor: Some(Cursor::Head),
        lt: PhantomData,
    };

    let _result = iter.next_unsafe();
    let _result = iter.next_unsafe();
    let _result = iter.next_unsafe();
}

