// Answer 0

#[test]
fn test_remove_extra_value_when_valid() {
    #[derive(Clone, Copy)]
    struct RawLink {
        next: usize,
        tail: usize,
    }

    #[derive(Clone, Copy)]
    struct ExtraValue<T> {
        prev: Link,
        next: Link,
        data: T,
    }

    #[derive(Clone, Copy)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    let mut raw_links: Vec<Option<RawLink>> = vec![Some(RawLink { next: 1, tail: 2 }), Some(RawLink { next: 2, tail: 0 }), Some(RawLink { next: 0, tail: 1 })];
    let mut extra_values: Vec<ExtraValue<u32>> = vec![
        ExtraValue { prev: Link::Entry(1), next: Link::Entry(2), data: 10 },
        ExtraValue { prev: Link::Extra(0), next: Link::Entry(0), data: 20 },
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(1), data: 30 },
    ];

    let result = remove_extra_value(&mut raw_links, &mut extra_values, 0);
    assert_eq!(result.data, 10);
    assert_eq!(extra_values.len(), 2);
    assert_eq!(raw_links[1].as_ref().map(|link| link.next), Some(2));
}

#[test]
#[should_panic]
fn test_remove_extra_value_with_invalid_index() {
    #[derive(Clone, Copy)]
    struct RawLink {
        next: usize,
        tail: usize,
    }

    #[derive(Clone, Copy)]
    struct ExtraValue<T> {
        prev: Link,
        next: Link,
        data: T,
    }

    #[derive(Clone, Copy)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    let mut raw_links: Vec<Option<RawLink>> = vec![Some(RawLink { next: 1, tail: 2 })];
    let mut extra_values: Vec<ExtraValue<u32>> = vec![ExtraValue { prev: Link::Entry(0), next: Link::Entry(0), data: 10 }];

    // This should panic since the index is out of bounds
    let _ = remove_extra_value(&mut raw_links, &mut extra_values, 1);
}

#[test]
#[should_panic]
fn test_remove_extra_value_with_invalid_prev() {
    #[derive(Clone, Copy)]
    struct RawLink {
        next: usize,
        tail: usize,
    }

    #[derive(Clone, Copy)]
    struct ExtraValue<T> {
        prev: Link,
        next: Link,
        data: T,
    }

    #[derive(Clone, Copy)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    let mut raw_links: Vec<Option<RawLink>> = vec![Some(RawLink { next: 1, tail: 2 })];
    let mut extra_values: Vec<ExtraValue<u32>> = vec![
        ExtraValue { prev: Link::Entry(1), next: Link::Entry(0), data: 10 },
    ];

    // This should panic since extra_values.len() <= prev is false
    let _ = remove_extra_value(&mut raw_links, &mut extra_values, 0);
}

