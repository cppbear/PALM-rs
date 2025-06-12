// Answer 0

#[test]
fn test_drain_all_extra_values() {
    #[derive(Debug)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    #[derive(Debug)]
    struct Bucket<T> {
        tail: Link,
        next: Link,
    }

    let raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { tail: Link::Extra(1), next: Link::Extra(2) }), 
                                                              Some(Bucket { tail: Link::Extra(2), next: Link::Entry(0) }),
                                                              None])));

    let mut extra_values = vec![
        ExtraValue { value: "First", prev: Link::Extra(1), next: Link::Extra(0) },
        ExtraValue { value: "Second", prev: Link::Extra(0), next: Link::Extra(2) },
        ExtraValue { value: "Third", prev: Link::Extra(2), next: Link::Extra(3) },
        ExtraValue { value: "Fourth", prev: Link::Extra(3), next: Link::Extra(4) },
        ExtraValue { value: "Fifth", prev: Link::Extra(4), next: Link::Entry(0) },
    ];

    let result = drain_all_extra_values(raw_links, &mut extra_values, 0);
    
    assert_eq!(result, vec!["First", "Second", "Third", "Fourth", "Fifth"]);
}

#[test]
#[should_panic]
fn test_drain_all_extra_values_invalid_index() {
    #[derive(Debug)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    #[derive(Debug)]
    struct Bucket<T> {
        tail: Link,
        next: Link,
    }

    let raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { tail: Link::Extra(1), next: Link::Extra(2) }), 
                                                              None])));

    let mut extra_values = vec![
        ExtraValue { value: "Invalid", prev: Link::Extra(1), next: Link::Extra(0) },
    ];

    // Pass an invalid head index, which should panic during the function execution.
    let _result = drain_all_extra_values(raw_links, &mut extra_values, 5);
}

