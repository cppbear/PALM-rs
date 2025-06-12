// Answer 0

#[test]
fn test_next_unsafe_with_cursor_none_and_no_entries() {
    struct TestHeaderMap {
        entries: Vec<Bucket<HeaderValue>>,
    }

    impl HeaderMap<HeaderValue> {
        fn new() -> Self {
            HeaderMap {
                mask: 0,
                indices: Box::new([]),
                entries: vec![],
                extra_values: vec![],
                danger: Danger::default(),
            }
        }
    }

    let mut map = TestHeaderMap {
        entries: vec![],
    };

    let mut iter = IterMut {
        map: &mut map as *mut _,
        entry: 0,
        cursor: None,
        lt: PhantomData,
    };

    assert_eq!(iter.next_unsafe(), None);
}

#[test]
fn test_next_unsafe_with_cursor_none_and_next_entry_out_of_bounds() {
    struct TestHeaderMap {
        entries: Vec<Bucket<HeaderValue>>,
    }

    let header_name = HeaderName { inner: Repr::Custom };
    let bucket = Bucket {
        hash: HashValue::default(),
        key: header_name,
        value: HeaderValue::default(),
        links: None,
    };

    let mut map = TestHeaderMap {
        entries: vec![bucket],
    };

    let mut iter = IterMut {
        map: &mut map as *mut _,
        entry: 0,  // This is the only entry
        cursor: None,
        lt: PhantomData,
    };

    // Move to a state where next_unsafe would run out of bounds
    iter.entry += 1;

    assert_eq!(iter.next_unsafe(), None);
}

