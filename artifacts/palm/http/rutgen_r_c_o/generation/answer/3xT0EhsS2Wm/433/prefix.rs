// Answer 0

#[test]
fn test_remove_extra_value_case1() {
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(1), next: Link::Entry(1) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 3, prev: Link::Entry(2), next: Link::Extra(1) },
    ];
    let raw_links = RawLinks(Box::into_raw(Box::new([
        Some(Links { next: 1, tail: 1 }),
        Some(Links { next: 2, tail: 2 }),
        None,
    ])));
    let idx = 2;

    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case2() {
    let mut extra_values = vec![
        ExtraValue { value: 'a', prev: Link::Entry(1), next: Link::Entry(1) },
        ExtraValue { value: 'b', prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 'c', prev: Link::Entry(2), next: Link::Entry(1) },
    ];
    let raw_links = RawLinks(Box::into_raw(Box::new([
        Some(Links { next: 1, tail: 1 }),
        None,
        Some(Links { next: 2, tail: 2 }),
    ])));
    let idx = 1;

    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case3() {
    let mut extra_values = vec![
        ExtraValue { value: 5.0, prev: Link::Entry(0), next: Link::Entry(1) },
        ExtraValue { value: 10.0, prev: Link::Entry(1), next: Link::Entry(0) },
    ];
    let raw_links = RawLinks(Box::into_raw(Box::new([
        Some(Links { next: 1, tail: 1 }),
        Some(Links { next: 0, tail: 0 }),
    ])));
    let idx = 0;

    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case4() {
    let mut extra_values = vec![
        ExtraValue { value: 'x', prev: Link::Entry(2), next: Link::Entry(3) },
        ExtraValue { value: 'y', prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 'z', prev: Link::Entry(1), next: Link::Entry(0) },
        ExtraValue { value: 'w', prev: Link::Entry(2), next: Link::Entry(3) },
    ];
    let raw_links = RawLinks(Box::into_raw(Box::new([
        Some(Links { next: 1, tail: 1 }),
        Some(Links { next: 2, tail: 2 }),
        None,
        None,
    ])));
    let idx = 3;

    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

