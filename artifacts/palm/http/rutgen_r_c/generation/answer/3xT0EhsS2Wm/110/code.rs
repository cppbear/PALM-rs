// Answer 0

#[test]
fn test_remove_extra_value_valid_case() {
    #[derive(Debug)]
    struct TestBucket {
        value: i32,
    }

    let mut extra_values = vec![
        ExtraValue {
            value: 1,
            prev: Link::Entry(0),
            next: Link::Entry(2),
        },
        ExtraValue {
            value: 2,
            prev: Link::Extra(0),
            next: Link::Entry(1),
        },
        ExtraValue {
            value: 3,
            prev: Link::Entry(1),
            next: Link::Extra(0),
        },
    ];
    
    let raw_links = RawLinks(Box::into_raw(Box::new([
        Some(TestBucket { value: 10 }),
        Some(TestBucket { value: 20 }),
        Some(TestBucket { value: 30 }),
    ])));

    let removed = remove_extra_value(raw_links, &mut extra_values, 0);
    assert_eq!(removed.value, 1);
    assert_eq!(extra_values.len(), 2);
    assert_eq!(extra_values[0].prev, Link::Entry(0));
}

#[test]
#[should_panic]
fn test_remove_extra_value_index_out_of_bounds() {
    #[derive(Debug)]
    struct TestBucket {
        value: i32,
    }

    let mut extra_values = vec![
        ExtraValue {
            value: 1,
            prev: Link::Entry(0),
            next: Link::Entry(1),
        },
    ];

    let raw_links = RawLinks(Box::into_raw(Box::new([
        Some(TestBucket { value: 10 }),
    ])));

    // This should panic since extra_values.len() is not greater than idx (1)
    remove_extra_value(raw_links, &mut extra_values, 1);
}

#[test]
#[should_panic]
fn test_remove_extra_value_invalid_next() {
    #[derive(Debug)]
    struct TestBucket {
        value: i32,
    }

    let mut extra_values = vec![
        ExtraValue {
            value: 1,
            prev: Link::Entry(0),
            next: Link::Entry(1),
        },
        ExtraValue {
            value: 2,
            prev: Link::Extra(0),
            next: Link::Entry(1),
        },
    ];

    let raw_links = RawLinks(Box::into_raw(Box::new([
        Some(TestBucket { value: 10 }),
        Some(TestBucket { value: 20 }),
    ])));

    // This should panic since it is set to have an invalid next
    extra_values[0].next = Link::Extra(2);
    remove_extra_value(raw_links, &mut extra_values, 0);
}

