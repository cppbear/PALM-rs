// Answer 0

#[test]
fn test_remove_extra_value_entry_entry() {
    let mut links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 2, tail: 1 }), Some(Bucket { next: 1, tail: 0 }), None])));
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 20, prev: Link::Entry(1), next: Link::Entry(1) },
    ];
    let idx = 0;

    let result = remove_extra_value(links, &mut extra_values, idx);

    assert_eq!(result.value, 10);
    assert_eq!(extra_values.len(), 1);
    assert_eq!(extra_values[0].prev, Link::Entry(1));
    assert_eq!(extra_values[0].next, Link::Entry(0));
}

#[test]
fn test_remove_extra_value_entry_extra() {
    let mut links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 2, tail: 1 }), None])));
    let mut extra_values = vec![
        ExtraValue { value: 30, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 40, prev: Link::Extra(1), next: Link::Entry(0) },
    ];
    let idx = 1;

    let result = remove_extra_value(links, &mut extra_values, idx);

    assert_eq!(result.value, 40);
    assert_eq!(extra_values.len(), 1);
    assert_eq!(extra_values[0].prev, Link::Entry(0));
    assert_eq!(extra_values[0].next, Link::Entry(0));
}

#[test]
fn test_remove_extra_value_extra_entry() {
    let mut links = RawLinks(Box::into_raw(Box::new([None, Some(Bucket { next: 0, tail: 1 })])));
    let mut extra_values = vec![
        ExtraValue { value: 50, prev: Link::Extra(0), next: Link::Entry(0) },
        ExtraValue { value: 60, prev: Link::Entry(0), next: Link::Extra(0) },
    ];
    let idx = 0;

    let result = remove_extra_value(links, &mut extra_values, idx);

    assert_eq!(result.value, 50);
    assert_eq!(extra_values.len(), 1);
    assert_eq!(extra_values[0].prev, Link::Entry(0));
    assert_eq!(extra_values[0].next, Link::Entry(0));
}

#[test]
fn test_remove_extra_value_extra_extra() {
    let mut links = RawLinks(Box::into_raw(Box::new([None, None])));
    let mut extra_values = vec![
        ExtraValue { value: 70, prev: Link::Extra(1), next: Link::Extra(0) },
        ExtraValue { value: 80, prev: Link::Extra(0), next: Link::Extra(1) },
    ];
    let idx = 1;

    let result = remove_extra_value(links, &mut extra_values, idx);

    assert_eq!(result.value, 80);
    assert_eq!(extra_values.len(), 1);
    assert_eq!(extra_values[0].prev, Link::Extra(0));
    assert_eq!(extra_values[0].next, Link::Extra(0));
}

