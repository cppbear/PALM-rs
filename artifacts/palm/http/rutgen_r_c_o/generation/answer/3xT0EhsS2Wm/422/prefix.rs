// Answer 0

#[test]
fn test_remove_extra_value_case_1() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 1, tail: 2 })])) as *mut [Bucket<i32>]);
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 20, prev: Link::Entry(1), next: Link::Extra(1) },
        ExtraValue { value: 30, prev: Link::Extra(0), next: Link::Extra(1) },
    ];
    let idx = 1;
    remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case_2() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 0, tail: 1 })])) as *mut [Bucket<i32>]);
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 20, prev: Link::Extra(0), next: Link::Entry(1) },
        ExtraValue { value: 30, prev: Link::Entry(1), next: Link::Entry(1) },
    ];
    let idx = 2;
    remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case_3() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 0, tail: 1 })])) as *mut [Bucket<i32>]);
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Entry(1) },
        ExtraValue { value: 20, prev: Link::Entry(1), next: Link::Extra(2) },
        ExtraValue { value: 30, prev: Link::Extra(1), next: Link::Extra(2) },
        ExtraValue { value: 40, prev: Link::Entry(3), next: Link::Entry(1) },
    ];
    let idx = 2;
    remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
#[should_panic]
fn test_remove_extra_value_case_invalid_index() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 1, tail: 2 })])) as *mut [Bucket<i32>]);
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Entry(0) },
    ];
    let idx = 1; // Invalid index
    remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case_multiple_removals() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 0, tail: 1 })])) as *mut [Bucket<i32>]);
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 2, prev: Link::Entry(1), next: Link::Entry(1) },
        ExtraValue { value: 3, prev: Link::Entry(2), next: Link::Entry(2) },
        ExtraValue { value: 4, prev: Link::Extra(1), next: Link::Extra(0) },
    ];
    for idx in (1..=3).rev() {
        remove_extra_value(raw_links, &mut extra_values, idx);
    }
}

