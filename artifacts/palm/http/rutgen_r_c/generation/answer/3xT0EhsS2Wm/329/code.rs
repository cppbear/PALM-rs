// Answer 0

#[test]
fn test_remove_extra_value_basic() {
    #[derive(Debug)]
    struct Bucket<T>(Option<T>);

    let mut raw_links = RawLinks(Box::into_raw(Box::new([Bucket(Some(0)), Bucket(Some(0))]))); // Simulate raw pointers
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Entry(1) },
        ExtraValue { value: 20, prev: Link::Extra(0), next: Link::Entry(1) },
    ];

    let idx = 0;
    let removed = remove_extra_value(raw_links, &mut extra_values, idx);

    assert_eq!(removed.value, 10);
    assert_eq!(extra_values.len(), 1);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_remove_extra_value_panic_len() {
    #[derive(Debug)]
    struct Bucket<T>(Option<T>);

    let mut raw_links = RawLinks(Box::into_raw(Box::new([Bucket(Some(0)), Bucket(Some(0))])));
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Extra(1) },
    ];

    let idx = 1; // This will cause len assertion to panic
    remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_remove_extra_value_panic_prev_len() {
    #[derive(Debug)]
    struct Bucket<T>(Option<T>);

    let mut raw_links = RawLinks(Box::into_raw(Box::new([Bucket(Some(0)), Bucket(None)]))); // This simulates a "None" state
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 20, prev: Link::Extra(0), next: Link::Entry(1) },
    ];

    let idx = 0; // Valid index
    remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_chain() {
    #[derive(Debug)]
    struct Bucket<T>(Option<T>);

    let mut raw_links = RawLinks(Box::into_raw(Box::new([Bucket(Some(0)), Bucket(Some(1)), Bucket(Some(2))]))); 
    let mut extra_values = vec![
        ExtraValue { value: 100, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 200, prev: Link::Extra(0), next: Link::Extra(2) },
        ExtraValue { value: 300, prev: Link::Extra(1), next: Link::Entry(1) },
    ];

    let idx = 1; 
    let removed = remove_extra_value(raw_links, &mut extra_values, idx);

    assert_eq!(removed.value, 200);
    assert_eq!(extra_values[0].next, Link::Extra(2));
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_remove_extra_value_corrupted_data() {
    #[derive(Debug)]
    struct Bucket<T>(Option<T>);

    let mut raw_links = RawLinks(Box::into_raw(Box::new([Bucket(Some(0)), Bucket(Some(1))])));
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 20, prev: Link::Entry(0), next: Link::Extra(1) },
    ];

    let idx = 0; 
    remove_extra_value(raw_links, &mut extra_values, idx);
    assert_eq!(extra_values.len(), 1); 
}

