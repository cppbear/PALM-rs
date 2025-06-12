// Answer 0

#[test]
fn test_drain_all_extra_values_multiple_entries() {
    let raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: Link::Extra(1), tail: 0 })]))); 
    let mut extra_values = vec![
        ExtraValue { value: "value0", prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: "value1", prev: Link::Extra(0), next: Link::Extra(2) },
        ExtraValue { value: "value2", prev: Link::Extra(1), next: Link::Entry(3) },
    ];
    let head = 0;

    drain_all_extra_values(raw_links, &mut extra_values, head);
}

#[test]
fn test_drain_all_extra_values_single_entry() {
    let raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: Link::Entry(1), tail: 0 })])));
    let mut extra_values = vec![
        ExtraValue { value: "value0", prev: Link::Entry(0), next: Link::Entry(1) },
    ];
    let head = 0;

    drain_all_extra_values(raw_links, &mut extra_values, head);
}

#[test]
fn test_drain_all_extra_values_empty_extra_values() {
    let raw_links = RawLinks(Box::into_raw(Box::new([None])));
    let mut extra_values = vec![];
    let head = 0;

    drain_all_extra_values(raw_links, &mut extra_values, head);
}

#[test]
fn test_drain_all_extra_values_edge_case() {
    let raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: Link::Extra(0), tail: 0 })])));
    let mut extra_values = vec![
        ExtraValue { value: "value0", prev: Link::Extra(0), next: Link::Extra(1) },
        ExtraValue { value: "value1", prev: Link::Entry(1), next: Link::Entry(2) },
    ];
    let head = 0;

    drain_all_extra_values(raw_links, &mut extra_values, head);
}

