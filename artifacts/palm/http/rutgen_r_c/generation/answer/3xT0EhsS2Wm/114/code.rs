// Answer 0

#[test]
fn test_remove_extra_value_with_valid_input() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 0, tail: 0 })])));

    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 20, prev: Link::Extra(0), next: Link::Entry(0) },
    ];

    let result = remove_extra_value(raw_links, &mut extra_values, 1);

    assert_eq!(result.value, 20);
    assert_eq!(extra_values.len(), 1);
}

#[test]
fn test_remove_extra_value_with_exchange() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 1, tail: 1 }), Some(Bucket { next: 0, tail: 0 })])));

    let mut extra_values = vec![
        ExtraValue { value: 30, prev: Link::Extra(1), next: Link::Entry(0) },
        ExtraValue { value: 40, prev: Link::Entry(0), next: Link::Extra(1) },
    ];

    let result = remove_extra_value(raw_links, &mut extra_values, 0);

    assert_eq!(result.value, 30);
    assert_eq!(extra_values.len(), 1);
} 

#[test]
#[should_panic]
fn test_remove_extra_value_with_invalid_index() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None])));

    let mut extra_values = vec![
        ExtraValue { value: 100, prev: Link::Entry(0), next: Link::Entry(0) },
    ];

    // This should panic as the index is out of bounds
    remove_extra_value(raw_links, &mut extra_values, 1);
}

