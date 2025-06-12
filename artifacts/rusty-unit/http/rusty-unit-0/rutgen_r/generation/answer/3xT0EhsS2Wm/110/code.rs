// Answer 0

#[test]
fn test_remove_extra_value_normal_case() {
    struct RawLinks<T> {
        links: Vec<Option<LinkEntry<T>>>,
    }

    struct LinkEntry<T> {
        next: usize,
        tail: usize,
        // ... other fields as necessary
    }

    enum Link {
        Entry(usize),
        Extra(usize),
    }

    struct ExtraValue<T> {
        prev: Link,
        next: Link,
        value: T, // Assuming an ExtraValue also holds some data of type T
    }

    let mut raw_links = RawLinks {
        links: vec![
            Some(LinkEntry { next: 1, tail: 1 }),
            Some(LinkEntry { next: 2, tail: 2 }),
            None, // Just an example, can be none
        ],
    };

    let mut extra_values = vec![
        ExtraValue {
            prev: Link::Entry(0),
            next: Link::Entry(1),
            value: "First",
        },
        ExtraValue {
            prev: Link::Extra(0),
            next: Link::Entry(2),
            value: "Second",
        },
        ExtraValue {
            prev: Link::Entry(1),
            next: Link::Extra(2),
            value: "Third",
        },
    ];

    let result = remove_extra_value(raw_links.links, &mut extra_values, 1);

    assert_eq!(result.value, "Second");
    assert_eq!(extra_values.len(), 2);
}

#[test]
#[should_panic]
fn test_remove_extra_value_out_of_bounds() {
    struct RawLinks<T> {
        links: Vec<Option<LinkEntry<T>>>,
    }

    struct LinkEntry<T> {
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

    let raw_links = RawLinks {
        links: vec![
            Some(LinkEntry { next: 1, tail: 1 }),
            Some(LinkEntry { next: 2, tail: 2 }),
        ],
    };

    let mut extra_values = vec![
        ExtraValue {
            prev: Link::Entry(0),
            next: Link::Entry(1),
            value: "Valid",
        },
    ];

    // Attempt to remove an extra value at an out-of-bounds index
    remove_extra_value(raw_links.links, &mut extra_values, 2);
}

#[test]
fn test_remove_extra_value_last_element() {
    struct RawLinks<T> {
        links: Vec<Option<LinkEntry<T>>>,
    }

    struct LinkEntry<T> {
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

    let mut raw_links = RawLinks {
        links: vec![
            Some(LinkEntry { next: 1, tail: 1 }),
            Some(LinkEntry { next: 2, tail: 2 }),
            None,
        ],
    };

    let mut extra_values = vec![
        ExtraValue {
            prev: Link::Entry(0),
            next: Link::Entry(1),
            value: "First",
        },
        ExtraValue {
            prev: Link::Entry(1),
            next: Link::Extra(0),
            value: "Second",
        },
    ];

    let result = remove_extra_value(raw_links.links, &mut extra_values, 1);

    assert_eq!(result.value, "Second");
    assert_eq!(extra_values.len(), 1);
    assert_eq!(extra_values[0].next, Link::Entry(0));
}

#[test]
#[should_panic]
fn test_remove_extra_value_index_equal_length() {
    struct RawLinks<T> {
        links: Vec<Option<LinkEntry<T>>>,
    }

    struct LinkEntry<T> {
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

    let raw_links = RawLinks {
        links: vec![
            Some(LinkEntry { next: 1, tail: 1 }),
            Some(LinkEntry { next: 2, tail: 2 }),
        ],
    };

    let mut extra_values = vec![
        ExtraValue {
            prev: Link::Entry(0),
            next: Link::Entry(1),
            value: "Valid",
        },
        ExtraValue {
            prev: Link::Entry(1),
            next: Link::Entry(2),
            value: "Invalid",
        },
    ];

    // Attempt to remove an extra value at the index equal to the length of extra_values
    remove_extra_value(raw_links.links, &mut extra_values, 2);
}

