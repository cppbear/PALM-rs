// Answer 0

#[test]
fn test_remove_extra_value_case_1() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 128])));
    let mut extra_values: Vec<ExtraValue<u32>> = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 3, prev: Link::Extra(1), next: Link::Entry(0) },
    ];
    let idx = 1;

    let extra = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case_2() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 128])));
    let mut extra_values: Vec<ExtraValue<u32>> = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Entry(1) },
        ExtraValue { value: 3, prev: Link::Entry(1), next: Link::Extra(2) },
        ExtraValue { value: 4, prev: Link::Extra(2), next: Link::Entry(0) },
    ];
    let idx = 2;

    let extra = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case_3() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 128])));
    let mut extra_values: Vec<ExtraValue<u32>> = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Extra(2) },
        ExtraValue { value: 2, prev: Link::Extra(0), next: Link::Extra(1) },
        ExtraValue { value: 3, prev: Link::Extra(1), next: Link::Extra(0) },
    ];
    let idx = 0;

    let extra = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
#[should_panic]
fn test_remove_extra_value_case_4() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 128])));
    let mut extra_values: Vec<ExtraValue<u32>> = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(1) },
    ];
    let idx = 1;  // Should panic due to extra_values.len() <= idx

    let extra = remove_extra_value(raw_links, &mut extra_values, idx);
}

