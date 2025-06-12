// Answer 0

#[derive(Debug)]
struct RawLinks<T>(Vec<Option<Links<T>>>);

#[derive(Debug)]
struct Links<T> {
    next: usize,
    tail: usize,
}

#[derive(Debug, Clone, Copy)]
enum Link {
    Entry(usize),
    Extra(usize),
}

#[derive(Debug)]
struct ExtraValue<T> {
    prev: Link,
    next: Link,
    value: T,
}

#[test]
fn test_remove_extra_value() {
    let mut raw_links = RawLinks(vec![
        Some(Links { next: 1, tail: 1 }),
        Some(Links { next: 2, tail: 2 }),
        None, // This entry simulates a broken link and should cause a panic when accessed.
    ]);

    let mut extra_values = vec![
        ExtraValue { prev: Link::Extra(1), next: Link::Entry(2), value: "val1" },
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(2), value: "val2" },
        ExtraValue { prev: Link::Extra(0), next: Link::Entry(1), value: "val3" },
    ];

    // This should panic since raw_links[2] is None
    let result = std::panic::catch_unwind(|| {
        remove_extra_value(raw_links, &mut extra_values, 1)
    });

    assert!(result.is_err());
}

#[test]
fn test_remove_extra_value_valid() {
    let mut raw_links = RawLinks(vec![
        Some(Links { next: 1, tail: 1 }),
        Some(Links { next: 2, tail: 2 }),
        Some(Links { next: 0, tail: 0 }),
    ]);

    let mut extra_values = vec![
        ExtraValue { prev: Link::Extra(1), next: Link::Entry(2), value: "val1" },
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(2), value: "val2" },
        ExtraValue { prev: Link::Extra(0), next: Link::Entry(1), value: "val3" },
    ];

    let removed_value = remove_extra_value(raw_links, &mut extra_values, 0);
    
    assert_eq!(removed_value.value, "val1");
    assert_eq!(extra_values.len(), 2);
    assert_eq!(extra_values[0].prev, Link::Extra(1));
    assert_eq!(extra_values[0].next, Link::Entry(1));
    assert_eq!(extra_values[1].prev, Link::Entry(0));
}

