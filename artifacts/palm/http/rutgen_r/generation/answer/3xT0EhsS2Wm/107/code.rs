// Answer 0

fn test_remove_extra_value() {
    struct RawLinks<T>(Vec<Option<Link<T>>>);
    struct ExtraValue<T> {
        prev: Link<T>,
        next: Link<T>,
    }
    #[derive(Clone, Copy)]
    enum Link<T> {
        Entry(usize),
        Extra(usize),
    }

    let mut raw_links = RawLinks(vec![
        Some(Link::<i32>::Entry(1)), 
        Some(Link::<i32>::Entry(0))
    ]);

    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(1), next: Link::Extra(1) }, // index 0
        ExtraValue { prev: Link::Extra(0), next: Link::Extra(0) }, // index 1
    ];

    // Test removing an extra value with a valid index
    let removed = remove_extra_value(
        raw_links,
        &mut extra_values,
        0
    );

    assert_eq!(removed.prev, Link::Entry(1));
    assert_eq!(removed.next, Link::Extra(1));
    assert_eq!(extra_values.len(), 1);
    assert!(matches!(extra_values[0].prev, Link::Extra(0)));
    assert!(matches!(extra_values[0].next, Link::Extra(0)));
}

fn test_remove_extra_value_boundary() {
    struct RawLinks<T>(Vec<Option<Link<T>>>);
    struct ExtraValue<T> {
        prev: Link<T>,
        next: Link<T>,
    }
    #[derive(Clone, Copy)]
    enum Link<T> {
        Entry(usize),
        Extra(usize),
    }

    let mut raw_links = RawLinks(vec![
        Some(Link::<i32>::Entry(1)), 
        Some(Link::<i32>::Entry(0))
    ]);

    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(1), next: Link::Extra(1) }, // index 0
        ExtraValue { prev: Link::Extra(0), next: Link::Extra(0) }, // index 1
    ];

    // Test removing the last remaining value
    let removed = remove_extra_value(
        raw_links,
        &mut extra_values,
        0
    );

    assert_eq!(removed.prev, Link::Entry(1));
    assert_eq!(removed.next, Link::Extra(1));
    assert!(extra_values.is_empty());
}

fn test_remove_extra_value_invalid_index() {
    struct RawLinks<T>(Vec<Option<Link<T>>>);
    struct ExtraValue<T> {
        prev: Link<T>,
        next: Link<T>,
    }
    #[derive(Clone, Copy)]
    enum Link<T> {
        Entry(usize),
        Extra(usize),
    }

    let mut raw_links = RawLinks(vec![
        Some(Link::<i32>::Entry(1)),
        Some(Link::<i32>::Entry(0)),
    ]);

    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(1), next: Link::Extra(1) }, // index 0
        ExtraValue { prev: Link::Extra(0), next: Link::Extra(0) }, // index 1
    ];

    // Test for panic on invalid index
    let result = std::panic::catch_unwind(|| {
        remove_extra_value(
            raw_links,
            &mut extra_values,
            2 // out of bounds index
        );
    });

    assert!(result.is_err());
}

