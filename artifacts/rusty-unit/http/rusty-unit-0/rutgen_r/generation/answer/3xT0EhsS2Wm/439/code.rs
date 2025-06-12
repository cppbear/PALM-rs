// Answer 0

#[test]
fn test_remove_extra_value_entry_entry() {
    struct RawLinks<T> {
        links: Vec<Option<LinkData<T>>>,
    }

    struct LinkData<T> {
        next: usize,
        tail: usize,
    }

    #[derive(Debug)]
    enum Link<T> {
        Entry(usize),
        Extra(usize),
    }

    struct ExtraValue<T> {
        prev: Link<T>,
        next: Link<T>,
    }

    let mut raw_links = RawLinks {
        links: vec![
            Some(LinkData { next: 1, tail: 2 }), // Entry 0
            Some(LinkData { next: 0, tail: 2 }), // Entry 1
            Some(LinkData { next: 1, tail: 1 }), // Extra 2
        ],
    };

    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Entry(1) }, // Extra value at index 0
        ExtraValue { prev: Link::Entry(1), next: Link::Extra(2) }, // Extra value at index 1
        ExtraValue { prev: Link::Extra(0), next: Link::Extra(1) }, // Extra value at index 2
    ];

    let extra_returned = remove_extra_value(raw_links.links, &mut extra_values, 0);

    assert_eq!(extra_returned.prev, Link::Entry(0));
    assert_eq!(extra_returned.next, Link::Entry(1));
}

#[test]
#[should_panic]
fn test_remove_extra_value_out_of_bounds() {
    struct RawLinks<T> {
        links: Vec<Option<LinkData<T>>>,
    }

    struct LinkData<T> {
        next: usize,
        tail: usize,
    }

    #[derive(Debug)]
    enum Link<T> {
        Entry(usize),
        Extra(usize),
    }

    struct ExtraValue<T> {
        prev: Link<T>,
        next: Link<T>,
    }

    let mut raw_links = RawLinks {
        links: vec![
            Some(LinkData { next: 1, tail: 2 }), // Entry 0
            Some(LinkData { next: 0, tail: 2 }), // Entry 1
        ],
    };

    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Entry(1) }, // Extra value at index 0
    ];

    let _ = remove_extra_value(raw_links.links, &mut extra_values, 1); // This should panic
}

#[test]
fn test_remove_extra_value_entry_extra() {
    struct RawLinks<T> {
        links: Vec<Option<LinkData<T>>>,
    }

    struct LinkData<T> {
        next: usize,
        tail: usize,
    }

    #[derive(Debug)]
    enum Link<T> {
        Entry(usize),
        Extra(usize),
    }

    struct ExtraValue<T> {
        prev: Link<T>,
        next: Link<T>,
    }

    let mut raw_links = RawLinks {
        links: vec![
            Some(LinkData { next: 1, tail: 2 }), // Entry 0
            None, // Entry 1 is empty
            Some(LinkData { next: 0, tail: 1 }), // Extra 2
        ],
    };

    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(2) }, // Extra value at index 0
        ExtraValue { prev: Link::Extra(2), next: Link::Entry(1) }, // Extra value at index 1
    ];

    let extra_returned = remove_extra_value(raw_links.links, &mut extra_values, 0);

    assert_eq!(extra_returned.prev, Link::Entry(0));
    assert_eq!(extra_returned.next, Link::Extra(2));
}

#[test]
fn test_remove_extra_value_extra_extra() {
    struct RawLinks<T> {
        links: Vec<Option<LinkData<T>>>,
    }

    struct LinkData<T> {
        next: usize,
        tail: usize,
    }

    #[derive(Debug)]
    enum Link<T> {
        Entry(usize),
        Extra(usize),
    }

    struct ExtraValue<T> {
        prev: Link<T>,
        next: Link<T>,
    }

    let mut raw_links = RawLinks {
        links: vec![
            None, // Entry 0 is empty
            None, // Entry 1 is empty
            Some(LinkData { next: 1, tail: 0 }), // Extra 2
            Some(LinkData { next: 0, tail: 1 }), // Extra 3
        ],
    };

    let mut extra_values = vec![
        ExtraValue { prev: Link::Extra(2), next: Link::Extra(3) }, // Extra value at index 0
        ExtraValue { prev: Link::Extra(3), next: Link::Extra(2) }, // Extra value at index 1
    ];

    let extra_returned = remove_extra_value(raw_links.links, &mut extra_values, 0);

    assert_eq!(extra_returned.prev, Link::Extra(2));
    assert_eq!(extra_returned.next, Link::Extra(3));
}

