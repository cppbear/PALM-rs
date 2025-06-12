// Answer 0

#[test]
fn test_next_unsafe_with_entry_and_extra_value() {
    #[derive(Debug, Clone)]
    struct MockHeaderMap {
        entries: Vec<Bucket<u32>>,
        extra_values: Vec<ExtraValue<u32>>,
    }

    let mut header_map = MockHeaderMap {
        entries: vec![
            Bucket {
                hash: 1,
                key: HeaderName { inner: Repr::Custom },
                value: 100,
                links: None,
            },
            Bucket {
                hash: 2,
                key: HeaderName { inner: Repr::Custom },
                value: 200,
                links: Some(Links { next: 1, tail: 1 }),
            },
        ],
        extra_values: vec![
            ExtraValue {
                value: 300,
                prev: Link::Entry(0),
                next: Link::Entry(1),
            },
            ExtraValue {
                value: 400,
                prev: Link::Extra(0),
                next: Link::Entry(0),
            },
        ],
    };

    let mut iter = IterMut {
        map: &mut header_map as *mut _,
        entry: 0,
        cursor: None,
        lt: PhantomData,
    };

    assert_eq!(iter.next_unsafe(), Some((&header_map.entries[1].key, &mut header_map.extra_values[0].value as *mut _)));
}

