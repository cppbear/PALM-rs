// Answer 0

#[test]
fn test_remove_extra_value_link_entry_to_entry() {
    struct RawLinks<T>(Vec<Option<Links<T>>>);
    struct Links<T> {
        next: usize,
        tail: usize,
    }
    
    #[derive(Clone)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    struct ExtraValue<T> {
        prev: Link,
        next: Link,
        value: T, // Assuming ExtraValue might have a value
    }

    let mut raw_links = RawLinks(vec![
        Some(Links { next: 1, tail: 1 }),
        Some(Links { next: 0, tail: 0 }),
    ]);
    
    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Entry(1), value: 42 },
        ExtraValue { prev: Link::Entry(1), next: Link::Entry(0), value: 24 },
    ];
    
    let idx = 0;

    let result = remove_extra_value(raw_links, &mut extra_values, idx);

    assert_eq!(result.value, 42);
    assert_eq!(extra_values.len(), 1);
    assert_eq!(extra_values[0].prev, Link::Extra(1));
    assert_eq!(extra_values[0].next, Link::Entry(1));
}

#[test]
fn test_remove_extra_value_link_extra_to_extra() {
    struct RawLinks<T>(Vec<Option<Links<T>>>);
    struct Links<T> {
        next: usize,
        tail: usize,
    }

    #[derive(Clone)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    struct ExtraValue<T> {
        prev: Link,
        next: Link,
        value: T,
    }

    let mut raw_links = RawLinks(vec![
        Some(Links { next: 1, tail: 1 }),
        Some(Links { next: 0, tail: 0 }),
        Some(Links { next: 1, tail: 1 }),
    ]);

    let mut extra_values = vec![
        ExtraValue { prev: Link::Extra(2), next: Link::Extra(1), value: 1 },
        ExtraValue { prev: Link::Extra(0), next: Link::Extra(2), value: 2 },
        ExtraValue { prev: Link::Extra(1), next: Link::Extra(0), value: 3 },
    ];

    let idx = 0;

    let result = remove_extra_value(raw_links, &mut extra_values, idx);

    assert_eq!(result.value, 1);
    assert_eq!(extra_values.len(), 2);
    assert_eq!(extra_values[0].prev, Link::Extra(1));
    assert_eq!(extra_values[0].next, Link::Extra(2));
}

#[test]
#[should_panic]
fn test_remove_extra_value_invalid_index() {
    struct RawLinks<T>(Vec<Option<Links<T>>>);
    struct Links<T> {
        next: usize,
        tail: usize,
    }

    #[derive(Clone)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    struct ExtraValue<T> {
        prev: Link,
        next: Link,
        value: T,
    }

    let mut raw_links = RawLinks(vec![
        Some(Links { next: 1, tail: 1 }),
    ]);
    
    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Entry(1), value: 42 },
    ];
    
    let idx = 1; // Invalid index

    // This should panic due to index out of bounds
    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_non_existent_extra() {
    struct RawLinks<T>(Vec<Option<Links<T>>>);
    struct Links<T> {
        next: usize,
        tail: usize,
    }

    #[derive(Clone)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    struct ExtraValue<T> {
        prev: Link,
        next: Link,
        value: T,
    }

    let mut raw_links = RawLinks(vec![
        Some(Links { next: 1, tail: 1 }),
        Some(Links { next: 0, tail: 0 }),
    ]);

    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(1), value: 42 },
        ExtraValue { prev: Link::Entry(1), next: Link::Entry(0), value: 24 },
    ];
    
    let idx = 1;

    let result = remove_extra_value(raw_links, &mut extra_values, idx);

    assert_eq!(result.value, 24);
    assert_eq!(extra_values.len(), 1);
    assert_eq!(extra_values[0].prev, Link::Entry(0));
    assert_eq!(extra_values[0].next, Link::Entry(0));
}

