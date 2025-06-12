// Answer 0

#[test]
fn test_remove_extra_value_entry_to_entry() {
    struct TestMap {
        indices: Vec<Option<(usize, usize)>>,
    }
    
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 1]))); 
    let mut extra_values = vec![
        ExtraValue { value: 0, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(0) },
    ];
    
    let result = remove_extra_value(raw_links, &mut extra_values, 0);
    
    assert_eq!(result.value, 0);
    assert!(extra_values.len() == 1);
}

#[test]
fn test_remove_extra_value_entry_to_extra() {
    struct TestMap {
        indices: Vec<Option<(usize, usize)>>,
    }
    
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 2])));
    let mut extra_values = vec![
        ExtraValue { value: 0, prev: Link::Entry(1), next: Link::Extra(1) },
        ExtraValue { value: 1, prev: Link::Extra(0), next: Link::Entry(1) },
    ];

    let result = remove_extra_value(raw_links, &mut extra_values, 0);
    
    assert_eq!(result.value, 0);
    assert!(extra_values.len() == 1);
}

#[test]
fn test_remove_extra_value_extra_to_entry() {
    struct TestMap {
        indices: Vec<Option<(usize, usize)>>,
    }
    
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 2])));
    let mut extra_values = vec![
        ExtraValue { value: 0, prev: Link::Extra(1), next: Link::Entry(1) },
        ExtraValue { value: 1, prev: Link::Entry(1), next: Link::Extra(0) },
    ];

    let result = remove_extra_value(raw_links, &mut extra_values, 0);
    
    assert_eq!(result.value, 0);
    assert!(extra_values.len() == 1);
}

#[test]
fn test_remove_extra_value_extra_to_extra() {
    struct TestMap {
        indices: Vec<Option<(usize, usize)>>,
    }
    
    let mut raw_links = RawLinks(Box::into_raw(Box::new([None; 2])));
    let mut extra_values = vec![
        ExtraValue { value: 0, prev: Link::Extra(1), next: Link::Extra(1) },
        ExtraValue { value: 1, prev: Link::Extra(0), next: Link::Extra(0) },
    ];

    let result = remove_extra_value(raw_links, &mut extra_values, 0);
    
    assert_eq!(result.value, 0);
    assert!(extra_values.len() == 1);
}

