// Answer 0

#[test]
fn test_remove_extra_value_case_1() {
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(0) },
    ];
    let raw_links = RawLinks(Box::into_raw(Box::new([Some(Links { next: 0, tail: 0 })])));

    let idx = 0;
    let result = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case_2() {
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Entry(0) },
    ];
    let raw_links = RawLinks(Box::into_raw(Box::new([Some(Links { next: 1, tail: 1 })])));

    let idx = 1;
    let result = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case_3() {
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 2, prev: Link::Extra(0), next: Link::Entry(0) },
    ];
    let raw_links = RawLinks(Box::into_raw(Box::new([Some(Links { next: 0, tail: 0 })])));

    let idx = 0;
    let result = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case_4() {
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Extra(0), next: Link::Extra(1) },
        ExtraValue { value: 2, prev: Link::Extra(1), next: Link::Entry(0) },
        ExtraValue { value: 3, prev: Link::Entry(1), next: Link::Entry(1) },
    ];
    let raw_links = RawLinks(Box::into_raw(Box::new([Some(Links { next: 1, tail: 0 }), Some(Links { next: 2, tail: 0 })])));

    let idx = 1;
    let result = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case_5() {
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Extra(2) },
        ExtraValue { value: 3, prev: Link::Extra(1), next: Link::Extra(0) },
    ];
    let raw_links = RawLinks(Box::into_raw(Box::new([
        Some(Links { next: 0, tail: 0 }),
        Some(Links { next: 1, tail: 0 }),
    ])));

    let idx = 2;
    let result = remove_extra_value(raw_links, &mut extra_values, idx);
}

