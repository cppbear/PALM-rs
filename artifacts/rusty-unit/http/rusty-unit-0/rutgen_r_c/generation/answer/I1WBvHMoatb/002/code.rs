// Answer 0

#[test]
fn test_drain_all_extra_values_with_multiple_extra_links() {
    #[derive(Debug)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    #[derive(Debug)]
    struct Bucket<T> {
        next: Link,
        tail: Link,
        value: T,
    }

    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Extra(3), next: Link::Extra(1) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Extra(2) },
        ExtraValue { value: 3, prev: Link::Extra(1), next: Link::Entry(4) },
        ExtraValue { value: 4, prev: Link::Extra(0), next: Link::Extra(3) },
        ExtraValue { value: 5, prev: Link::Entry(0), next: Link::Extra(0) },
    ];

    let raw_links = RawLinks(Box::into_raw(Box::new([
        Some(Bucket { next: Link::Extra(1), tail: Link::Entry(4), value: 10 }),
        Some(Bucket { next: Link::Extra(2), tail: Link::Entry(3), value: 20 }),
        Some(Bucket { next: Link::Entry(4), tail: Link::Extra(0), value: 30 }),
        Some(Bucket { next: Link::Extra(3), tail: Link::Entry(0), value: 40 }),
        Some(Bucket { next: Link::Extra(0), tail: Link::Extra(1), value: 50 }),
    ])));

    let result = drain_all_extra_values(raw_links, &mut extra_values, 0);

    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
#[should_panic]
fn test_drain_all_extra_values_with_invalid_index() {
    #[derive(Debug)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    #[derive(Debug)]
    struct Bucket<T> {
        next: Link,
        tail: Link,
        value: T,
    }

    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Extra(0), next: Link::Extra(1) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Extra(2) },
    ];

    let raw_links = RawLinks(Box::into_raw(Box::new([
        Some(Bucket { next: Link::Extra(1), tail: Link::Entry(2), value: 10 }),
        Some(Bucket { next: Link::Entry(2), tail: Link::Extra(0), value: 20 }),
    ])));

    drain_all_extra_values(raw_links, &mut extra_values, 999); // Invalid index
}

#[test]
fn test_drain_all_extra_values_with_single_link() {
    #[derive(Debug)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    #[derive(Debug)]
    struct Bucket<T> {
        next: Link,
        tail: Link,
        value: T,
    }

    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Extra(0), next: Link::Entry(2) },
    ];

    let raw_links = RawLinks(Box::into_raw(Box::new([
        Some(Bucket { next: Link::Entry(2), tail: Link::Entry(2), value: 10 }),
    ])));

    let result = drain_all_extra_values(raw_links, &mut extra_values, 0);

    assert_eq!(result, vec![1]);
}

