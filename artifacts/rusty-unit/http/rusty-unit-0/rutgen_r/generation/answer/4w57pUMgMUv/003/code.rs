// Answer 0

#[test]
fn test_next_unsafe_with_valid_inputs() {
    struct HeaderMap {
        entries: Vec<Entry>,
        extra_values: Vec<ExtraValue>,
    }

    struct Entry {
        key: HeaderName,
        value: u32,
        links: Option<Link>,
    }

    struct ExtraValue {
        value: u64,
        next: Link,
    }

    enum Link {
        Entry(usize),
        Extra(usize),
    }

    #[derive(Clone)]
    struct HeaderName(String);

    struct Cursor {
        cursor: Option<CursorType>,
        entry: usize,
        map: *mut HeaderMap,
    }

    enum CursorType {
        Head,
        Values(usize),
    }

    let mut map = HeaderMap {
        entries: vec![
            Entry {
                key: HeaderName("key1".to_string()),
                value: 0,
                links: Some(Link::Extra(0)),
            },
            Entry {
                key: HeaderName("key2".to_string()),
                value: 1,
                links: Some(Link::Entry(0)),
            },
        ],
        extra_values: vec![
            ExtraValue {
                value: 0,
                next: Link::Entry(1),
            },
            ExtraValue {
                value: 1,
                next: Link::Entry(2),
            },
        ],
    };

    let mut cursor = Cursor {
        cursor: None,
        entry: 0,
        map: &mut map as *mut _,
    };

    let result = cursor.next_unsafe();
    assert!(result.is_some());

    if let Some((key, value_ptr)) = result {
        assert_eq!(key.0, "key1");
        unsafe {
            assert_eq!(*value_ptr, 0);
        }
    }
}

#[test]
fn test_next_unsafe_with_no_extra_values() {
    struct HeaderMap {
        entries: Vec<Entry>,
        extra_values: Vec<ExtraValue>,
    }

    struct Entry {
        key: HeaderName,
        value: u32,
        links: Option<Link>,
    }

    struct ExtraValue {
        value: u64,
        next: Link,
    }

    enum Link {
        Entry(usize),
        Extra(usize),
    }

    #[derive(Clone)]
    struct HeaderName(String);

    struct Cursor {
        cursor: Option<CursorType>,
        entry: usize,
        map: *mut HeaderMap,
    }

    enum CursorType {
        Head,
        Values(usize),
    }

    let mut map = HeaderMap {
        entries: vec![
            Entry {
                key: HeaderName("key1".to_string()),
                value: 0,
                links: Some(Link::Entry(0)),
            },
        ],
        extra_values: vec![],
    };

    let mut cursor = Cursor {
        cursor: None,
        entry: 0,
        map: &mut map as *mut _,
    };

    let result = cursor.next_unsafe();
    assert!(result.is_some());

    if let Some((key, value_ptr)) = result {
        assert_eq!(key.0, "key1");
        unsafe {
            assert_eq!(*value_ptr, 0);
        }
    }
}

