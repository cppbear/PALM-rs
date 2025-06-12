// Answer 0

#[test]
fn test_remove_extra_value_valid_entry_pair() {
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(1) },
        ExtraValue { value: 2, prev: Link::Entry(1), next: Link::Entry(0) },
        ExtraValue { value: 3, prev: Link::Entry(0), next: Link::Extra(2) }
    ];
    
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None, None, None])));
    
    remove_extra_value(raw_links, &mut extra_values, 1);
}

#[test]
fn test_remove_extra_value_valid_extra_pair() {
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Extra(0), next: Link::Entry(1) },
        ExtraValue { value: 2, prev: Link::Entry(1), next: Link::Extra(2) },
        ExtraValue { value: 3, prev: Link::Extra(2), next: Link::Extra(0) }
    ];
    
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None, None, None])));
    
    remove_extra_value(raw_links, &mut extra_values, 0);
}

#[test]
fn test_remove_extra_value_entry_to_extra() {
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(1), next: Link::Extra(2) },
        ExtraValue { value: 2, prev: Link::Extra(2), next: Link::Extra(0) },
        ExtraValue { value: 3, prev: Link::Entry(0), next: Link::Entry(1) }
    ];
    
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None, None, None])));
    
    remove_extra_value(raw_links, &mut extra_values, 2);
}

#[test]
fn test_remove_extra_value_extra_to_entry() {
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Extra(0), next: Link::Entry(2) },
        ExtraValue { value: 2, prev: Link::Entry(1), next: Link::Extra(1) },
        ExtraValue { value: 3, prev: Link::Entry(2), next: Link::Extra(0) }
    ];
    
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None, None, None])));
    
    remove_extra_value(raw_links, &mut extra_values, 1);
}

#[test]
fn test_remove_extra_value_extra_pair_valid_indices() {
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Extra(1), next: Link::Extra(0) },
        ExtraValue { value: 2, prev: Link::Extra(0), next: Link::Extra(2) },
        ExtraValue { value: 3, prev: Link::Extra(2), next: Link::Entry(1) }
    ];
    
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None, None, None])));
    
    remove_extra_value(raw_links, &mut extra_values, 0);
}

