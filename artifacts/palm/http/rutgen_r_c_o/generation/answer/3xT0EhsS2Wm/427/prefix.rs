// Answer 0

#[test]
fn test_remove_extra_value_basic_case() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 1, tail: 1 }); 5]))); 
    let mut extra_values = vec![
        ExtraValue { value: 0, prev: Link::Entry(1), next: Link::Entry(1) },
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Entry(0) },
    ];
    let idx = 0;
    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_multiple_cases() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 2, tail: 2 }); 6]))); 
    let mut extra_values = vec![
        ExtraValue { value: 0, prev: Link::Entry(1), next: Link::Entry(1) },
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Extra(2) },
        ExtraValue { value: 2, prev: Link::Extra(1), next: Link::Entry(0) },
        ExtraValue { value: 3, prev: Link::Entry(1), next: Link::Entry(0) },
    ];
    let idx = 1;
    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_edge_case() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 3, tail: 3 }); 8]))); 
    let mut extra_values = vec![
        ExtraValue { value: 0, prev: Link::Entry(2), next: Link::Entry(2) },
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 3, prev: Link::Entry(1), next: Link::Entry(1) },
        ExtraValue { value: 4, prev: Link::Entry(0), next: Link::Entry(3) },
    ];
    let idx = 2;
    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_swap_case() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 1, tail: 1 }); 10]))); 
    let mut extra_values = vec![
        ExtraValue { value: 0, prev: Link::Entry(1), next: Link::Entry(1) },
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Entry(1) },
        ExtraValue { value: 3, prev: Link::Extra(1), next: Link::Entry(2) },
        ExtraValue { value: 4, prev: Link::Entry(2), next: Link::Entry(1) },
    ];
    let idx = 3;
    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

