// Answer 0

#[test]
#[should_panic]
fn test_remove_extra_value_panic_on_index_out_of_bounds() {
    struct Dummy;

    let raw_links = RawLinks(ptr::null_mut());
    let mut extra_values = vec![ExtraValue { value: Dummy, prev: Link::Entry(0), next: Link::Entry(0) }];
    
    remove_extra_value(raw_links, &mut extra_values, 1);
}

#[test]
fn test_remove_extra_value_valid_case() {
    struct Dummy;

    let mut extra_values = vec![
        ExtraValue { value: Dummy, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: Dummy, prev: Link::Entry(0), next: Link::Entry(0) },
    ];
    let raw_links = RawLinks(ptr::null_mut());

    let result = remove_extra_value(raw_links, &mut extra_values, 0);
}

