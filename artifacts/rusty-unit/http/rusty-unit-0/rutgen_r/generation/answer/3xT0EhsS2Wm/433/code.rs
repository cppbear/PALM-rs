// Answer 0

#[test]
fn test_remove_extra_value_entry_entry() {
    struct RawLinks<T>(Vec<Option<LinkNode<T>>>);
    struct LinkNode<T> {
        next: usize,
        tail: usize,
    }

    #[derive(Clone, Copy)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    struct ExtraValue<T> {
        prev: Link,
        next: Link,
    }

    // Setup data for the test
    let mut raw_links = RawLinks(vec![Some(LinkNode { next: 1, tail: 0 }), None]);
    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { prev: Link::Extra(0), next: Link::Extra(0) },
    ];

    // Execute the function
    let removed = remove_extra_value(raw_links, &mut extra_values, 0);

    // Validate the results
    assert_eq!(removed.prev, Link::Entry(0));
    assert_eq!(removed.next, Link::Entry(0));
}

#[test]
fn test_remove_extra_value_extra_entry() {
    struct RawLinks<T>(Vec<Option<LinkNode<T>>>);
    struct LinkNode<T> {
        next: usize,
        tail: usize,
    }

    #[derive(Clone, Copy)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    struct ExtraValue<T> {
        prev: Link,
        next: Link,
    }

    let mut raw_links = RawLinks(vec![Some(LinkNode { next: 1, tail: 0 }), None]);
    let mut extra_values = vec![
        ExtraValue { prev: Link::Extra(0), next: Link::Entry(1) },
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(0) },
    ];

    let removed = remove_extra_value(raw_links, &mut extra_values, 0);

    assert_eq!(removed.prev, Link::Extra(0));
    assert_eq!(removed.next, Link::Entry(1));
}

#[test]
fn test_remove_extra_value_extra_extra() {
    struct RawLinks<T>(Vec<Option<LinkNode<T>>>);
    struct LinkNode<T> {
        next: usize,
        tail: usize,
    }

    #[derive(Clone, Copy)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    struct ExtraValue<T> {
        prev: Link,
        next: Link,
    }

    let mut raw_links = RawLinks(vec![None, None]);
    let mut extra_values = vec![
        ExtraValue { prev: Link::Extra(1), next: Link::Extra(0) },
        ExtraValue { prev: Link::Extra(0), next: Link::Extra(1) },
    ];

    let removed = remove_extra_value(raw_links, &mut extra_values, 0);

    assert_eq!(removed.prev, Link::Extra(1));
    assert_eq!(removed.next, Link::Extra(0));
}

