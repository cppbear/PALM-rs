// Answer 0

#[test]
fn test_remove_extra_value_valid_case() {
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(1), next: Link::Entry(3) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Entry(2) },
        ExtraValue { value: 3, prev: Link::Entry(2), next: Link::Entry(1) },
        ExtraValue { value: 4, prev: Link::Entry(1), next: Link::Entry(0) },
    ];
    
    let mut raw_links: RawLinks<u32> = RawLinks(Box::into_raw(Box::new([None, Some(Links { next: 2, tail: 3 }), Some(Links { next: 3, tail: 1 }), None])));

    let idx = 1;
    remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_removal() {
    let mut extra_values = vec![
        ExtraValue { value: 5, prev: Link::Entry(2), next: Link::Entry(1) },
        ExtraValue { value: 6, prev: Link::Entry(1), next: Link::Entry(2) },
        ExtraValue { value: 7, prev: Link::Entry(0), next: Link::Entry(3) },
        ExtraValue { value: 8, prev: Link::Entry(1), next: Link::Entry(0) },
    ];

    let mut raw_links: RawLinks<u32> = RawLinks(Box::into_raw(Box::new([Some(Links { next: 1, tail: 2 }), Some(Links { next: 0, tail: 3 }), None, None])));

    let idx = 2;
    remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_edge_case() {
    let mut extra_values = vec![
        ExtraValue { value: 9, prev: Link::Entry(1), next: Link::Entry(1) },
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Entry(0) },
    ];
    
    let mut raw_links: RawLinks<u32> = RawLinks(Box::into_raw(Box::new([Some(Links { next: 0, tail: 0 }), None])));

    let idx = 0;
    remove_extra_value(raw_links, &mut extra_values, idx);
}

