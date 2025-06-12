// Answer 0

#[test]
fn test_remove_extra_value_entry_to_entry() {
    struct RawLinks<T>(Vec<Option<Links<T>>>);
    struct Links<T> {
        next: usize,
        tail: usize,
    }
    struct ExtraValue<T> {
        prev: Link,
        next: Link,
    }
    
    #[derive(Clone, Copy)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    let mut raw_links = RawLinks(vec![
        Some(Links { next: 1, tail: 1 }),
        Some(Links { next: 0, tail: 0 }),
    ]);
    
    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Entry(1) },
        ExtraValue { prev: Link::Entry(0), next: Link::Entry(0) },
    ];

    let result = remove_extra_value(raw_links.0.clone(), &mut extra_values, 1);
    
    assert_eq!(result.next, Link::Entry(0));
    assert_eq!(result.prev, Link::Entry(0));
}

#[test]
fn test_remove_extra_value_entry_to_extra() {
    struct RawLinks<T>(Vec<Option<Links<T>>>);
    struct Links<T> {
        next: usize,
        tail: usize,
    }
    struct ExtraValue<T> {
        prev: Link,
        next: Link,
    }
    
    #[derive(Clone, Copy)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    let mut raw_links = RawLinks(vec![
        Some(Links { next: 1, tail: 1 }),
        Some(Links { next: 2, tail: 2 }),
        Some(Links { next: 0, tail: 0 }),
    ]);
    
    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { prev: Link::Extra(1), next: Link::Entry(2) },
        ExtraValue { prev: Link::Entry(2), next: Link::Extra(0) },
    ];

    let result = remove_extra_value(raw_links.0.clone(), &mut extra_values, 1);
    
    assert_eq!(result.prev, Link::Entry(0));
    assert_eq!(result.next, Link::Entry(2));
}

#[test]
fn test_remove_extra_value_extra_to_entry() {
    struct RawLinks<T>(Vec<Option<Links<T>>>);
    struct Links<T> {
        next: usize,
        tail: usize,
    }
    struct ExtraValue<T> {
        prev: Link,
        next: Link,
    }
    
    #[derive(Clone, Copy)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    let mut raw_links = RawLinks(vec![
        Some(Links { next: 1, tail: 1 }),
        Some(Links { next: 0, tail: 0 }),
    ]);

    let mut extra_values = vec![
        ExtraValue { prev: Link::Extra(0), next: Link::Entry(1) },
        ExtraValue { prev: Link::Entry(1), next: Link::Extra(0) },
    ];

    let result = remove_extra_value(raw_links.0.clone(), &mut extra_values, 0);
    
    assert_eq!(result.prev, Link::Entry(1));
    assert_eq!(result.next, Link::Extra(0));
}

#[test]
fn test_remove_extra_value_extra_to_extra() {
    struct RawLinks<T>(Vec<Option<Links<T>>>);
    struct Links<T> {
        next: usize,
        tail: usize,
    }
    struct ExtraValue<T> {
        prev: Link,
        next: Link,
    }
    
    #[derive(Clone, Copy)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    let mut raw_links = RawLinks(vec![
        Some(Links { next: 1, tail: 1 }),
        Some(Links { next: 2, tail: 2 }),
        Some(Links { next: 0, tail: 0 }),
    ]);

    let mut extra_values = vec![
        ExtraValue { prev: Link::Extra(0), next: Link::Extra(1) },
        ExtraValue { prev: Link::Extra(1), next: Link::Extra(2) },
        ExtraValue { prev: Link::Extra(2), next: Link::Extra(0) },
    ];

    let result = remove_extra_value(raw_links.0.clone(), &mut extra_values, 1);
    
    assert_eq!(result.prev, Link::Extra(0));
    assert_eq!(result.next, Link::Extra(2));
}

