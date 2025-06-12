// Answer 0

#[test]
fn test_remove_extra_value_success_case() {
    #[derive(Debug)]
    struct Bucket<T>(Option<T>);

    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 3, prev: Link::Extra(0), next: Link::Extra(1) },
    ];

    let raw_links = RawLinks(Box::into_raw(Box::new([
        Some(Bucket::<i32>(Some(100))), 
        Some(Bucket::<i32>(Some(200))), 
        None
    ])));

    let result = remove_extra_value(raw_links, &mut extra_values, 2);
    
    assert_eq!(result.value, 3);
    assert_eq!(extra_values.len(), 2);
    assert!(matches!(extra_values[0].next, Link::Entry(1)));
}

#[test]
fn test_remove_extra_value_entry_link_case() {
    #[derive(Debug)]
    struct Bucket<T>(Option<T>);

    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 2, prev: Link::Entry(1), next: Link::Entry(1) },
    ];

    let raw_links = RawLinks(Box::into_raw(Box::new([
        Some(Bucket::<i32>(Some(100))), 
        Some(Bucket::<i32>(Some(200)))
    ])));

    let result = remove_extra_value(raw_links, &mut extra_values, 1);

    assert_eq!(result.value, 2);
    assert_eq!(extra_values.len(), 1);
    assert!(matches!(extra_values[0].next, Link::Entry(0)));
}

#[test]
fn test_remove_extra_value_extra_link_case() {
    #[derive(Debug)]
    struct Bucket<T>(Option<T>);

    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Entry(0) },
    ];

    let raw_links = RawLinks(Box::into_raw(Box::new([
        Some(Bucket::<i32>(Some(100))),
        Some(Bucket::<i32>(Some(200)))
    ])));

    let result = remove_extra_value(raw_links, &mut extra_values, 0);

    assert_eq!(result.value, 1);
    assert_eq!(extra_values.len(), 1);
    assert!(matches!(extra_values[0].next, Link::Entry(0)));
}

#[should_panic] 
#[test]
fn test_remove_extra_value_invalid_index_panics() {
    #[derive(Debug)]
    struct Bucket<T>(Option<T>);

    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(0) },
    ];

    let raw_links = RawLinks(Box::into_raw(Box::new([
        Some(Bucket::<i32>(Some(100)))
    ])));

    let _ = remove_extra_value(raw_links, &mut extra_values, 1);
}

#[test]
fn test_remove_extra_value_non_existing_extra_case() {
    #[derive(Debug)]
    struct Bucket<T>(Option<T>);

    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Extra(1), next: Link::Entry(0) },
    ];

    let raw_links = RawLinks(Box::into_raw(Box::new([
        None,
        Some(Bucket::<i32>(Some(200)))
    ])));

    let result = remove_extra_value(raw_links, &mut extra_values, 0);
    
    assert_eq!(result.value, 1);
    assert_eq!(extra_values.len(), 0);
}

