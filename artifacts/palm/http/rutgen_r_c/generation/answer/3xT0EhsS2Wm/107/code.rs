// Answer 0

#[test]
fn test_remove_extra_value_removes_correctly() {
    struct DummyValue {
        value: i32,
    }
    let mut extra_values: Vec<ExtraValue<DummyValue>> = vec![
        ExtraValue { value: DummyValue { value: 1 }, prev: Link::Extra(1), next: Link::Extra(2) },
        ExtraValue { value: DummyValue { value: 2 }, prev: Link::Entry(0), next: Link::Extra(0) },
        ExtraValue { value: DummyValue { value: 3 }, prev: Link::Extra(0), next: Link::Entry(0) },
    ];
    
    let raw_links = RawLinks(ptr::null_mut());
    
    let removed_extra = remove_extra_value(raw_links, &mut extra_values, 1);
    
    assert_eq!(removed_extra.value.value, 2);
    assert_eq!(extra_values.len(), 2);
    assert_eq!(extra_values[0].next, Link::Extra(1));
    assert_eq!(extra_values[1].prev, Link::Entry(0));
}

#[test]
#[should_panic]
fn test_remove_extra_value_index_out_of_bounds() {
    struct DummyValue {
        value: i32,
    }
    let mut extra_values: Vec<ExtraValue<DummyValue>> = vec![
        ExtraValue { value: DummyValue { value: 1 }, prev: Link::Extra(1), next: Link::Extra(2) },
    ];
    
    let raw_links = RawLinks(ptr::null_mut());
    
    let _ = remove_extra_value(raw_links, &mut extra_values, 1);
}

#[test]
fn test_remove_extra_value_updates_links() {
    struct DummyValue {
        value: i32,
    }
    let mut extra_values: Vec<ExtraValue<DummyValue>> = vec![
        ExtraValue { value: DummyValue { value: 1 }, prev: Link::Extra(1), next: Link::Entry(2) },
        ExtraValue { value: DummyValue { value: 2 }, prev: Link::Entry(0), next: Link::Extra(0) },
        ExtraValue { value: DummyValue { value: 3 }, prev: Link::Extra(1), next: Link::Extra(0) },
    ];
    
    let raw_links = RawLinks(ptr::null_mut());
    
    let removed_extra = remove_extra_value(raw_links, &mut extra_values, 1);
    
    assert_eq!(removed_extra.value.value, 2);
    assert_eq!(extra_values.len(), 2);
    assert_eq!(extra_values[0].next, Link::Entry(1));
}

#[test]
fn test_remove_extra_value_no_displacement() {
    struct DummyValue {
        value: i32,
    }
    let mut extra_values: Vec<ExtraValue<DummyValue>> = vec![
        ExtraValue { value: DummyValue { value: 1 }, prev: Link::Extra(1), next: Link::Extra(2) },
        ExtraValue { value: DummyValue { value: 2 }, prev: Link::Entry(0), next: Link::Entry(0) },
    ];
    
    let raw_links = RawLinks(ptr::null_mut());
    
    let removed_extra = remove_extra_value(raw_links, &mut extra_values, 0);
    
    assert_eq!(removed_extra.value.value, 1);
    assert_eq!(extra_values.len(), 1);
    assert_eq!(extra_values[0].next, Link::Entry(0));
}

