// Answer 0

#[test]
fn test_remove_extra_value_valid_case() {
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

    #[derive(Clone)]
    struct ExtraValue<T> {
        prev: Link,
        next: Link,
    }

    let mut raw_links = RawLinks(vec![
        Some(Links { next: 1, tail: 2 }),
        Some(Links { next: 2, tail: 0 }),
        Some(Links { next: 0, tail: 1 }),
    ]);
    
    let mut extra_values = vec![
        ExtraValue { prev: Link::Extra(1), next: Link::Entry(0) },
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(2) },
        ExtraValue { prev: Link::Entry(1), next: Link::Entry(1) },
    ];

    let idx = 1;
    let extra = remove_extra_value(raw_links, &mut extra_values, idx);

    assert_eq!(extra.prev, Link::Entry(0));
    assert_eq!(extra.next, Link::Extra(2));
}

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

    #[derive(Clone)]
    struct ExtraValue<T> {
        prev: Link,
        next: Link,
    }

    let mut raw_links = RawLinks(vec![
        Some(Links { next: 1, tail: 2 }),
        Some(Links { next: 2, tail: 0 }),
        Some(Links { next: 0, tail: 1 }),
    ]);
    
    let mut extra_values = vec![
        ExtraValue { prev: Link::Extra(1), next: Link::Entry(0) },
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(2) },
        ExtraValue { prev: Link::Entry(1), next: Link::Entry(1) },
    ];

    let idx = 5; // Invalid index
    remove_extra_value(raw_links, &mut extra_values, idx);
}

#[should_panic]
fn test_remove_extra_value_no_raw_link_present() {
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

    #[derive(Clone)]
    struct ExtraValue<T> {
        prev: Link,
        next: Link,
    }

    let mut raw_links = RawLinks(vec![
        None,
        None,
        None,
    ]);

    let mut extra_values = vec![
        ExtraValue { prev: Link::Extra(1), next: Link::Entry(0) },
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(2) },
        ExtraValue { prev: Link::Entry(1), next: Link::Entry(1) },
    ];

    let idx = 1;
    remove_extra_value(raw_links, &mut extra_values, idx);
}

