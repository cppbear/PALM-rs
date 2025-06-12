// Answer 0

#[derive(Debug)]
struct RawLinks<T>(Vec<Option<LinkNode<T>>>);

#[derive(Debug)]
struct LinkNode<T> {
    next: usize,
    tail: usize,
    value: T,
}

#[derive(Debug)]
struct ExtraValue<T> {
    prev: Link<T>,
    next: Link<T>,
}

#[derive(Debug)]
enum Link<T> {
    Entry(usize),
    Extra(usize),
}

#[test]
fn test_remove_extra_value_removes_correct_value() {
    let mut raw_links = RawLinks(vec![
        Some(LinkNode { next: 1, tail: 1, value: "A" }),
        Some(LinkNode { next: 2, tail: 2, value: "B" }),
        Some(LinkNode { next: 3, tail: 3, value: "C" }),
    ]);
    
    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { prev: Link::Extra(0), next: Link::Entry(1) },
    ];
    
    let removed_extra = remove_extra_value(raw_links, &mut extra_values, 0);
    assert_eq!(removed_extra.prev, Link::Entry(0));
    assert_eq!(removed_extra.next, Link::Extra(1));
    assert_eq!(extra_values.len(), 1);
}

#[test]
fn test_remove_extra_value_updates_links() {
    let mut raw_links = RawLinks(vec![
        Some(LinkNode { next: 1, tail: 1, value: "A" }),
        Some(LinkNode { next: 2, tail: 2, value: "B" }),
        Some(LinkNode { next: 3, tail: 3, value: "C" }),
    ]);

    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { prev: Link::Extra(0), next: Link::Entry(1) },
    ];

    let _removed_extra = remove_extra_value(raw_links, &mut extra_values, 1);
    assert_eq!(extra_values.len(), 1);
}

#[test]
#[should_panic]
fn test_remove_extra_value_panic_on_invalid_index() {
    let mut raw_links = RawLinks(vec![
        Some(LinkNode { next: 1, tail: 1, value: "A" }),
        Some(LinkNode { next: 2, tail: 2, value: "B" }),
    ]);

    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(1) },
    ];

    remove_extra_value(raw_links, &mut extra_values, 2);
}

