// Answer 0

#[test]
fn test_remove_extra_value_link_entry_to_link_extra() {
    struct RawLinks<T>(Vec<Option<Links<T>>>);
    struct Links<T> {
        next: usize,
        tail: usize,
    }

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
        Some(Links { next: 1, tail: 0 }),
        Some(Links { next: 2, tail: 1 }),
        None,
        Some(Links { next: 3, tail: 2 }),
    ]);

    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(1), value: "a" },
        ExtraValue { prev: Link::Extra(0), next: Link::Extra(2), value: "b" },
        ExtraValue { prev: Link::Entry(3), next: Link::Entry(2), value: "c" },
    ];

    let removed = remove_extra_value(&mut raw_links, &mut extra_values, 1);

    assert_eq!(removed.value, "b");
    assert_eq!(extra_values.len(), 2);
    assert_eq!(extra_values[0].next, Link::Extra(1));
}

#[test]
#[should_panic]
fn test_remove_extra_value_idx_out_of_bounds() {
    struct RawLinks<T>(Vec<Option<Links<T>>>);
    struct Links<T> {
        next: usize,
        tail: usize,
    }

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
        Some(Links { next: 1, tail: 0 }),
    ]);

    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(0), value: "a" }
    ];

    remove_extra_value(&mut raw_links, &mut extra_values, 1);
}

#[test]
fn test_remove_extra_value_link_extra_to_link_entry() {
    struct RawLinks<T>(Vec<Option<Links<T>>>);
    struct Links<T> {
        next: usize,
        tail: usize,
    }

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
        Some(Links { next: 1, tail: 0 }),
        None,
        Some(Links { next: 2, tail: 1 }),
    ]);

    let mut extra_values = vec![
        ExtraValue { prev: Link::Extra(0), next: Link::Entry(1), value: "a" },
        ExtraValue { prev: Link::Extra(1), next: Link::Extra(2), value: "b" },
        ExtraValue { prev: Link::Entry(2), next: Link::Entry(1), value: "c" },
    ];

    let removed = remove_extra_value(&mut raw_links, &mut extra_values, 0);

    assert_eq!(removed.value, "a");
    assert_eq!(extra_values.len(), 2);
}

#[test]
fn test_remove_extra_value_multiple_operations() {
    struct RawLinks<T>(Vec<Option<Links<T>>>);
    struct Links<T> {
        next: usize,
        tail: usize,
    }

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
        Some(Links { next: 1, tail: 0 }),
        Some(Links { next: 2, tail: 1 }),
        Some(Links { next: 3, tail: 2 }),
        None,
    ]);

    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(1), value: "First" },
        ExtraValue { prev: Link::Extra(0), next: Link::Entry(2), value: "Second" },
        ExtraValue { prev: Link::Entry(3), next: Link::Entry(2), value: "Third" },
    ];

    let removed_first = remove_extra_value(&mut raw_links, &mut extra_values, 1);
    assert_eq!(removed_first.value, "Second");

    let removed_second = remove_extra_value(&mut raw_links, &mut extra_values, 0);
    assert_eq!(removed_second.value, "First");

    assert_eq!(extra_values.len(), 1);
}

