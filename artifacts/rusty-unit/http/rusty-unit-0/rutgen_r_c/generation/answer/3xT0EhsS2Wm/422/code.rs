// Answer 0

#[test]
fn test_remove_extra_value_at_entry_prev_next_entry() {
    let mut links = vec![Some(Bucket { next: 1, tail: 1 })];
    let raw_links = RawLinks(links.as_mut_ptr());

    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 20, prev: Link::Entry(0), next: Link::Entry(0) },
    ];

    let result = remove_extra_value(raw_links, &mut extra_values, 0);

    assert_eq!(result.value, 10);
    assert_eq!(extra_values.len(), 1);
    assert!(matches!(extra_values[0].prev, Link::Entry(0)));
}

#[test]
fn test_remove_extra_value_at_entry_prev_extra() {
    let mut links = vec![Some(Bucket { next: 1, tail: 1 })];
    let raw_links = RawLinks(links.as_mut_ptr());

    let mut extra_values = vec![
        ExtraValue { value: 30, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 40, prev: Link::Extra(0), next: Link::Entry(0) },
    ];

    let result = remove_extra_value(raw_links, &mut extra_values, 0);

    assert_eq!(result.value, 30);
    assert_eq!(extra_values.len(), 1);
    assert!(matches!(extra_values[0].next, Link::Extra(0)));
}

#[test]
fn test_remove_extra_value_at_extra_prev_next_entry() {
    let mut links = vec![Some(Bucket { next: 1, tail: 2 })];
    let raw_links = RawLinks(links.as_mut_ptr());

    let mut extra_values = vec![
        ExtraValue { value: 50, prev: Link::Extra(0), next: Link::Entry(1) },
        ExtraValue { value: 60, prev: Link::Extra(0), next: Link::Entry(0) },
    ];

    let result = remove_extra_value(raw_links, &mut extra_values, 1);

    assert_eq!(result.value, 60);
    assert_eq!(extra_values.len(), 1);
    assert!(matches!(extra_values[0].prev, Link::Extra(0)));
}

#[test]
fn test_remove_extra_value_at_extra_prev_extra() {
    let mut links = vec![Some(Bucket { next: 1, tail: 1 })];
    let raw_links = RawLinks(links.as_mut_ptr());

    let mut extra_values = vec![
        ExtraValue { value: 70, prev: Link::Extra(1), next: Link::Extra(2) },
        ExtraValue { value: 80, prev: Link::Extra(0), next: Link::Extra(2) },
        ExtraValue { value: 90, prev: Link::Extra(1), next: Link::Entry(0) },
    ];

    let result = remove_extra_value(raw_links, &mut extra_values, 1);

    assert_eq!(result.value, 80);
    assert_eq!(extra_values.len(), 2);
    assert!(matches!(extra_values[0].next, Link::Extra(1)));
}

#[test]
#[should_panic]
fn test_remove_extra_value_out_of_bounds() {
    let mut links = vec![Some(Bucket { next: 1, tail: 1 })];
    let raw_links = RawLinks(links.as_mut_ptr());

    let extra_values = vec![
        ExtraValue { value: 100, prev: Link::Entry(0), next: Link::Extra(0) },
    ];

    let _ = remove_extra_value(raw_links, &mut extra_values, 1); // Out of bounds
}

#[test]
#[should_panic]
fn test_remove_extra_value_invalid_prev() {
    let mut links = vec![Some(Bucket { next: 1, tail: 1 })];
    let raw_links = RawLinks(links.as_mut_ptr());

    let mut extra_values = vec![
        ExtraValue { value: 110, prev: Link::Entry(1), next: Link::Entry(0) },
        ExtraValue { value: 120, prev: Link::Entry(0), next: Link::Entry(0) },
    ];

    let _ = remove_extra_value(raw_links, &mut extra_values, 0); // Invalid prev
}

