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
        Some(Links { next: 1, tail: 0 }),
        None,
        Some(Links { next: 2, tail: 1 }),
    ]);
    
    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Entry(1), value: "A" },
        ExtraValue { prev: Link::Extra(0), next: Link::Extra(2), value: "B" },
        ExtraValue { prev: Link::Extra(1), next: Link::Entry(1), value: "C" },
    ];

    let idx = 1; // Valid index to remove
    let removed_extra_value = remove_extra_value(raw_links, &mut extra_values, idx);
    
    assert_eq!(removed_extra_value.value, "B");
    assert_eq!(extra_values.len(), 2);
    assert!(matches!(extra_values[0].next, Link::Extra(1))); // Check link integrity post removal
    assert!(matches!(extra_values[1].prev, Link::Extra(0))); // Check link integrity post removal
}

#[test]
#[should_panic]
fn test_remove_invalid_index() {
    let mut raw_links = RawLinks(vec![
        Some(Links { next: 1, tail: 0 }),
    ]);
    
    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Entry(1), value: "A" },
    ];

    let idx = 2; // Invalid index to remove
    remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_first_extra_value() {
    let mut raw_links = RawLinks(vec![
        Some(Links { next: 1, tail: 0 }),
        Some(Links { next: 2, tail: 1 }),
        None,
    ]);
    
    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Entry(1), value: "A" },
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(2), value: "B" },
        ExtraValue { prev: Link::Extra(1), next: Link::Entry(1), value: "C" },
    ];

    let idx = 0; // Removing the first ExtraValue
    let removed_extra_value = remove_extra_value(raw_links, &mut extra_values, idx);
    
    assert_eq!(removed_extra_value.value, "A");
    assert_eq!(extra_values.len(), 2);
    assert!(matches!(extra_values[0].prev, Link::Extra(1))); // Validate link integrity
}

#[test]
fn test_remove_last_extra_value() {
    let mut raw_links = RawLinks(vec![
        Some(Links { next: 1, tail: 0 }),
        Some(Links { next: 2, tail: 1 }),
        None,
    ]);
    
    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(1), next: Link::Entry(2), value: "A" },
        ExtraValue { prev: Link::Entry(0), next: Link::Entry(1), value: "B" },
        ExtraValue { prev: Link::Extra(1), next: Link::Entry(0), value: "C" },
    ];

    let idx = 2; // Removing last ExtraValue
    let removed_extra_value = remove_extra_value(raw_links, &mut extra_values, idx);
    
    assert_eq!(removed_extra_value.value, "C");
    assert_eq!(extra_values.len(), 2);
    assert!(matches!(extra_values[1].next, Link::Entry(0))); // Validate link integrity
}

