// Answer 0

#[test]
fn test_remove_extra_value_first_link() {
    #[derive(Debug)]
    struct RawLinks<T>(Vec<Option<LinkData<T>>>);
    
    #[derive(Debug)]
    struct LinkData<T> {
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
    }

    let mut raw_links = RawLinks(vec![
        Some(LinkData { next: 1, tail: 0 }),
        Some(LinkData { next: 2, tail: 1 }),
        None,
    ]);
    
    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { prev: Link::Extra(0), next: Link::Entry(2) },
    ];

    let result = remove_extra_value(
        raw_links,
        &mut extra_values,
        1
    );

    assert_eq!(result.prev, Link::Extra(0));
    assert_eq!(result.next, Link::Entry(2));
    assert_eq!(extra_values.len(), 1);
}

#[test]
#[should_panic]
fn test_remove_extra_value_panic_due_to_invalid_index() {
    #[derive(Debug)]
    struct RawLinks<T>(Vec<Option<LinkData<T>>>);
    
    #[derive(Debug)]
    struct LinkData<T> {
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
    }

    let mut raw_links = RawLinks(vec![
        Some(LinkData { next: 1, tail: 0 }),
    ]);
    
    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(1) },
    ];

    remove_extra_value(raw_links, &mut extra_values, 1);
}

#[test]
#[should_panic]
fn test_remove_extra_value_panic_due_to_raw_links_none() {
    #[derive(Debug)]
    struct RawLinks<T>(Vec<Option<LinkData<T>>>);
    
    #[derive(Debug)]
    struct LinkData<T> {
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
    }

    let mut raw_links = RawLinks(vec![
        None,
        Some(LinkData { next: 1, tail: 0 }),
    ]);
    
    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(1) },
    ];

    remove_extra_value(raw_links, &mut extra_values, 0);
}

#[test]
fn test_remove_extra_value_middle_link() {
    #[derive(Debug)]
    struct RawLinks<T>(Vec<Option<LinkData<T>>>);
    
    #[derive(Debug)]
    struct LinkData<T> {
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
    }

    let mut raw_links = RawLinks(vec![
        Some(LinkData { next: 1, tail: 0 }),
        Some(LinkData { next: 2, tail: 1 }),
        Some(LinkData { next: 3, tail: 2 }),
        None,
    ]);
    
    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { prev: Link::Extra(0), next: Link::Extra(2) },
        ExtraValue { prev: Link::Extra(1), next: Link::Extra(3) },
    ];

    let result = remove_extra_value(
        raw_links,
        &mut extra_values,
        1
    );

    assert_eq!(result.prev, Link::Extra(0));
    assert_eq!(result.next, Link::Extra(2));
    assert_eq!(extra_values.len(), 2);
}

