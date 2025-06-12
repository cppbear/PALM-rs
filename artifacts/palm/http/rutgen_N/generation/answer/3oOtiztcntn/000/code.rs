// Answer 0

#[derive(Debug)]
struct Bucket<T> {
    links: Option<Links>,
}

#[derive(Debug)]
struct Links {
    next: usize,
    tail: usize,
}

#[derive(Debug)]
struct ExtraValue<T> {
    value: T,
    prev: Link,
    next: Link,
}

#[derive(Debug)]
enum Link {
    Extra(usize),
    Entry(usize),
}

#[test]
fn test_append_value_with_existing_links() {
    let entry_idx = 0;
    let mut entry = Bucket {
        links: Some(Links { next: 1, tail: 0 }),
    };
    let mut extra: Vec<ExtraValue<i32>> = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Entry(0) },
    ];
    let value = 20;

    append_value(entry_idx, &mut entry, &mut extra, value);

    assert_eq!(extra.len(), 2);
    assert_eq!(extra[1].value, 20);
    assert_eq!(extra[1].prev, Link::Extra(0));
    assert_eq!(extra[1].next, Link::Entry(0));
    assert_eq!(entry.links.as_ref().unwrap().tail, 1);
}

#[test]
fn test_append_value_without_existing_links() {
    let entry_idx = 0;
    let mut entry = Bucket { links: None };
    let mut extra: Vec<ExtraValue<i32>> = vec![];
    let value = 30;

    append_value(entry_idx, &mut entry, &mut extra, value);

    assert_eq!(extra.len(), 1);
    assert_eq!(extra[0].value, 30);
    assert_eq!(extra[0].prev, Link::Entry(0));
    assert_eq!(extra[0].next, Link::Entry(0));
    assert!(entry.links.is_some());
    assert_eq!(entry.links.as_ref().unwrap().next, 0);
    assert_eq!(entry.links.as_ref().unwrap().tail, 0);
}

