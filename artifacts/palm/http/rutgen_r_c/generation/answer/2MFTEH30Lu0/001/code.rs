// Answer 0

#[test]
fn test_remove_extra_value_success() {
    #[derive(Debug)]
    struct TestHeaderValue;
    
    let mut extra_values = vec![
        ExtraValue { value: TestHeaderValue, prev: Link::Entry(0), next: Link::Entry(1) },
        ExtraValue { value: TestHeaderValue, prev: Link::Extra(0), next: Link::Extra(2) },
        ExtraValue { value: TestHeaderValue, prev: Link::Extra(1), next: Link::Entry(2) }
    ];

    let mut map = HeaderMap {
        mask: 0,
        indices: Box::new([]),
        entries: vec![],
        extra_values,
        danger: Danger::Green,
    };

    let removed_value = map.remove_extra_value(1);
    assert_eq!(removed_value.prev, Link::Extra(0));
    assert_eq!(removed_value.next, Link::Extra(2));
}

#[test]
#[should_panic]
fn test_remove_extra_value_invalid_index() {
    #[derive(Debug)]
    struct TestHeaderValue;

    let mut extra_values = vec![
        ExtraValue { value: TestHeaderValue, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: TestHeaderValue, prev: Link::Extra(0), next: Link::Entry(2) },
        ExtraValue { value: TestHeaderValue, prev: Link::Extra(1), next: Link::Entry(2) }
    ];

    let mut map = HeaderMap {
        mask: 0,
        indices: Box::new([]),
        entries: vec![],
        extra_values,
        danger: Danger::Green,
    };

    // Attempting to remove an extra value at an index that is out of bounds
    let _ = map.remove_extra_value(5);
}

