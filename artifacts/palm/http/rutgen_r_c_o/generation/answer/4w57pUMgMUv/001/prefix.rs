// Answer 0

#[test]
fn test_next_unsafe_cursor_none_entry_out_of_bounds() {
    let mut header_map = HeaderMap {
        mask: 0,
        indices: Box::from([]),
        entries: vec![
            Bucket {
                hash: HashValue::default(),
                key: HeaderName { inner: Repr::default() },
                value: HeaderValue::default(),
                links: None,
            }
        ],
        extra_values: vec![],
        danger: Danger::default(),
    };

    let mut iter = IterMut {
        map: &mut header_map as *mut _,
        entry: 0,
        cursor: None,
        lt: PhantomData,
    };

    let result = iter.next_unsafe();
}

