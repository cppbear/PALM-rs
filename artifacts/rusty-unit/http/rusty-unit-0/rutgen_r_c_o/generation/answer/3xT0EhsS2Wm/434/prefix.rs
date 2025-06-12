// Answer 0

#[test]
fn test_remove_extra_value_valid_case() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 1, tail: 2 }), Some(Bucket { next: 0, tail: 1 })])));
    let mut extra_values = vec![
        ExtraValue { value: "value1", prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: "value2", prev: Link::Entry(0), next: Link::Entry(1) },
        ExtraValue { value: "value3", prev: Link::Entry(1), next: Link::Extra(2) },
    ];
    let idx = 1;
    remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_edge_case() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 0, tail: 0 }), None])));
    let mut extra_values = vec![
        ExtraValue { value: "value1", prev: Link::Entry(0), next: Link::Entry(0) },
    ];
    let idx = 0;
    remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
#[should_panic]
fn test_remove_extra_value_invalid_index() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 1, tail: 2 })])));
    let mut extra_values = vec![
        ExtraValue { value: "value1", prev: Link::Entry(0), next: Link::Entry(0) },
    ];
    let idx = 1; // Invalid index
    remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_entry_entry() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 0, tail: 0 }), Some(Bucket { next: 1, tail: 1 })])));
    let mut extra_values = vec![
        ExtraValue { value: "value1", prev: Link::Entry(0), next: Link::Entry(1) },
        ExtraValue { value: "value2", prev: Link::Entry(0), next: Link::Entry(0) },
    ];
    let idx = 0;
    remove_extra_value(raw_links, &mut extra_values, idx);
}

