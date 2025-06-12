// Answer 0

#[test]
fn test_remove_extra_value_edge_case_1() {
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 20, prev: Link::Extra(0), next: Link::Entry(2) },
        ExtraValue { value: 30, prev: Link::Entry(1), next: Link::Extra(3) },
        ExtraValue { value: 40, prev: Link::Extra(2), next: Link::Entry(4) },
        ExtraValue { value: 50, prev: Link::Entry(3), next: Link::Extra(4) },
    ];
    let raw_links = RawLinks(ptr::null_mut());
    let idx = 4;

    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_edge_case_2() {
    let mut extra_values = vec![
        ExtraValue { value: 5, prev: Link::Entry(3), next: Link::Extra(4) },
        ExtraValue { value: 15, prev: Link::Extra(1), next: Link::Entry(2) },
        ExtraValue { value: 25, prev: Link::Entry(0), next: Link::Extra(3) },
        ExtraValue { value: 35, prev: Link::Extra(2), next: Link::Entry(4) },
        ExtraValue { value: 45, prev: Link::Entry(3), next: Link::Extra(5) },
        ExtraValue { value: 55, prev: Link::Extra(4), next: Link::Entry(5) },
    ];
    let raw_links = RawLinks(ptr::null_mut());
    let idx = 5;

    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_small_vec() {
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(1) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Entry(0) },
    ];
    let raw_links = RawLinks(ptr::null_mut());
    let idx = 1;

    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_first_element() {
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Extra(1), next: Link::Entry(1) },
        ExtraValue { value: 20, prev: Link::Entry(0), next: Link::Extra(2) },
        ExtraValue { value: 30, prev: Link::Extra(1), next: Link::Entry(3) },
    ];
    let raw_links = RawLinks(ptr::null_mut());
    let idx = 0;

    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_last_element() {
    let mut extra_values = vec![
        ExtraValue { value: 100, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 200, prev: Link::Entry(1), next: Link::Extra(2) },
        ExtraValue { value: 300, prev: Link::Extra(1), next: Link::Extra(3) },
    ];
    let raw_links = RawLinks(ptr::null_mut());
    let idx = 2;

    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

