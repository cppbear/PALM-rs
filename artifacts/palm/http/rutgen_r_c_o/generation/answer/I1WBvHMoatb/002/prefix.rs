// Answer 0

#[test]
fn test_drain_all_extra_values_with_single_extra_value() {
    let mut extra_values = vec![ExtraValue { value: 42, prev: Link::Extra(0), next: Link::Extra(1) }];
    let raw_links = RawLinks(Box::into_raw(Box::new([None; 1])));
    let result = drain_all_extra_values(raw_links, &mut extra_values, 0);
}

#[test]
fn test_drain_all_extra_values_with_multiple_extra_values() {
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Extra(0), next: Link::Extra(1) },
        ExtraValue { value: 2, prev: Link::Extra(1), next: Link::Extra(2) },
        ExtraValue { value: 3, prev: Link::Extra(2), next: Link::Entry(0) }
    ];
    let raw_links = RawLinks(Box::into_raw(Box::new([None; 3])));
    let result = drain_all_extra_values(raw_links, &mut extra_values, 0);
}

#[test]
fn test_drain_all_extra_values_with_edge_case() {
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Extra(0), next: Link::Entry(0) },
    ];
    let raw_links = RawLinks(Box::into_raw(Box::new([None; 1])));
    let result = drain_all_extra_values(raw_links, &mut extra_values, 0);
}

#[test]
fn test_drain_all_extra_values_with_large_data() {
    let mut extra_values = (0..100).map(|i| ExtraValue { value: i, prev: Link::Extra(i as usize), next: Link::Extra(i + 1) }).collect::<Vec<_>>();
    extra_values.push(ExtraValue { value: 100, prev: Link::Extra(99), next: Link::Entry(0) }); // Loop to entry
    let raw_links = RawLinks(Box::into_raw(Box::new([None; 101])));
    let result = drain_all_extra_values(raw_links, &mut extra_values, 0);
}

#[test]
#[should_panic]
fn test_drain_all_extra_values_with_invalid_head() {
    let mut extra_values = vec![ExtraValue { value: 5, prev: Link::Extra(0), next: Link::Extra(1) }];
    let raw_links = RawLinks(Box::into_raw(Box::new([None; 1])));
    let _result = drain_all_extra_values(raw_links, &mut extra_values, 999); // Invalid head
}

