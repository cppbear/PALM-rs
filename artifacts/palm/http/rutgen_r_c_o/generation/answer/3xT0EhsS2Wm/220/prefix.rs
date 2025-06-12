// Answer 0

#[test]
fn test_remove_extra_value_valid_case() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 32768])));
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Extra(1), next: Link::Entry(2) },
        ExtraValue { value: 2, prev: Link::Extra(0), next: Link::Entry(3) },
        ExtraValue { value: 3, prev: Link::Entry(2), next: Link::Extra(1) },
        ExtraValue { value: 4, prev: Link::Extra(3), next: Link::Extra(2) },
    ];
    
    let idx = 1; // Valid index
    let result = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
#[should_panic]
fn test_remove_extra_value_invalid_index() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 32768])));
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Extra(1), next: Link::Entry(2) },
    ];
    
    let idx = 1; // Invalid index, since extra_values.len() is not greater than idx
    let result = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_edge_case() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 32768])));
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Extra(2), next: Link::Entry(3) },
        ExtraValue { value: 2, prev: Link::Entry(1), next: Link::Entry(4) },
        ExtraValue { value: 3, prev: Link::Extra(0), next: Link::Entry(5) },
        ExtraValue { value: 4, prev: Link::Entry(1), next: Link::Extra(2) },
    ];
    
    let idx = 2; // Valid index
    let result = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_with_none_raw_links() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 32768])));
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Extra(0), next: Link::Entry(1) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Entry(2) },
    ];
    
    let idx = 0; // Valid index
    let result = remove_extra_value(raw_links, &mut extra_values, idx);
}

