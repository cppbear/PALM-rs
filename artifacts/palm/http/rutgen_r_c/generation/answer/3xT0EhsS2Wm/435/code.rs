// Answer 0

#[test]
fn test_remove_extra_value_with_link_entry_entry() {
    #[derive(Debug)]
    struct Bucket<T>(Option<T>);
    
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Bucket(Some(1)), Bucket(Some(1)), Bucket(None)])));
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 20, prev: Link::Entry(0), next: Link::Entry(0) }
    ];
    
    let idx = 0;
    
    let result = remove_extra_value(raw_links, &mut extra_values, idx);
    
    assert_eq!(result.value, 10);
    assert_eq!(extra_values.len(), 1);
    assert_eq!(extra_values[0].prev, Link::Entry(0));
    assert_eq!(extra_values[0].next, Link::Entry(0));
}

#[test]
fn test_remove_extra_value_with_link_entry_extra() {
    #[derive(Debug)]
    struct Bucket<T>(Option<T>);
    
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Bucket(Some(1)), Bucket(Some(1)), Bucket(None)])));
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 20, prev: Link::Extra(0), next: Link::Entry(0) }
    ];
    
    let idx = 0;
    
    let result = remove_extra_value(raw_links, &mut extra_values, idx);
    
    assert_eq!(result.value, 10);
    assert_eq!(extra_values.len(), 1);
    assert_eq!(extra_values[0].prev, Link::Extra(0));
    assert_eq!(extra_values[0].next, Link::Entry(0));
}

#[test]
fn test_remove_extra_value_with_link_extra_entry() {
    #[derive(Debug)]
    struct Bucket<T>(Option<T>);
    
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Bucket(Some(1)), Bucket(Some(1)), Bucket(None)])));
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Extra(0), next: Link::Entry(1) },
        ExtraValue { value: 20, prev: Link::Entry(0), next: Link::Extra(0) }
    ];
    
    let idx = 1;
    
    let result = remove_extra_value(raw_links, &mut extra_values, idx);
    
    assert_eq!(result.value, 20);
    assert_eq!(extra_values.len(), 1);
    assert_eq!(extra_values[0].prev, Link::Entry(0));
    assert_eq!(extra_values[0].next, Link::Extra(0));
}

#[test]
fn test_remove_extra_value_with_link_extra_extra() {
    #[derive(Debug)]
    struct Bucket<T>(Option<T>);
    
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Bucket(Some(1)), Bucket(Some(1)), Bucket(None)])));
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Extra(0), next: Link::Extra(1) },
        ExtraValue { value: 20, prev: Link::Extra(0), next: Link::Entry(0) }
    ];
    
    let idx = 0;
    
    let result = remove_extra_value(raw_links, &mut extra_values, idx);
    
    assert_eq!(result.value, 10);
    assert_eq!(extra_values.len(), 1);
    assert_eq!(extra_values[0].next, Link::Entry(0));
    assert_eq!(extra_values[0].prev, Link::Extra(0));
}

