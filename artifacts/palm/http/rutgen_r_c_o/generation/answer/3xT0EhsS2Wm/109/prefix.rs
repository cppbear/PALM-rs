// Answer 0

#[test]
fn test_remove_extra_value_normal_case() {
    let raw_links = RawLinks(ptr::null_mut());
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 2, prev: Link::Extra(0), next: Link::Extra(2) },
        ExtraValue { value: 3, prev: Link::Extra(1), next: Link::Entry(2) },
    ];
    let idx = 1;
    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_last_element() {
    let raw_links = RawLinks(ptr::null_mut());
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 2, prev: Link::Extra(0), next: Link::Extra(2) },
        ExtraValue { value: 3, prev: Link::Entry(2), next: Link::Entry(2) },
    ];
    let idx = 2;
    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_edge_case() {
    let raw_links = RawLinks(ptr::null_mut());
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(1) },
        ExtraValue { value: 2, prev: Link::Entry(1), next: Link::Entry(0) },
    ];
    let idx = 0;
    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
#[should_panic]
fn test_remove_extra_value_invalid_index() {
    let raw_links = RawLinks(ptr::null_mut());
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Extra(1) },
    ];
    let idx = 1; // This index is out of bounds
    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
} 

#[test]
fn test_remove_extra_value_two_elements() {
    let raw_links = RawLinks(ptr::null_mut());
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 20, prev: Link::Extra(0), next: Link::Entry(1) },
    ];
    let idx = 0;
    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

