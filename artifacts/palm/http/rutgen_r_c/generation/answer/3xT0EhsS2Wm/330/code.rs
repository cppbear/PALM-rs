// Answer 0

#[test]
fn test_remove_extra_value_entry_entry() {
    #[derive(Debug)]
    struct TestBucket;

    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(TestBucket), Some(TestBucket)])));
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 20, prev: Link::Entry(1), next: Link::Entry(1) },
    ];

    let idx = 0;

    let result = remove_extra_value(raw_links, &mut extra_values, idx);

    assert_eq!(result.value, 10);
    assert!(extra_values.len() == 1);
}

#[test]
fn test_remove_extra_value_entry_extra() {
    #[derive(Debug)]
    struct TestBucket;

    let mut raw_links = RawLinks(Box::into_raw(Box::new([None, Some(TestBucket), None])));
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 20, prev: Link::Extra(0), next: Link::Entry(1) },
    ];

    let idx = 0;

    let result = remove_extra_value(raw_links, &mut extra_values, idx);

    assert_eq!(result.value, 10);
    assert_eq!(extra_values.len(), 1);
}

#[test]
fn test_remove_extra_value_extra_entry() {
    #[derive(Debug)]
    struct TestBucket;

    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(TestBucket), None])));
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Extra(0), next: Link::Entry(1) },
        ExtraValue { value: 20, prev: Link::Entry(0), next: Link::Extra(0) },
    ];

    let idx = 0;

    let result = remove_extra_value(raw_links, &mut extra_values, idx);

    assert_eq!(result.value, 10);
    assert_eq!(extra_values.len(), 1);
}

#[test]
fn test_remove_extra_value_extra_extra() {
    #[derive(Debug)]
    struct TestBucket;

    let mut raw_links = RawLinks(Box::into_raw(Box::new([None, None])));
    let mut extra_values = vec![
        ExtraValue { value: 10, prev: Link::Extra(0), next: Link::Extra(1) },
        ExtraValue { value: 20, prev: Link::Extra(1), next: Link::Extra(0) },
    ];

    let idx = 0;

    let result = remove_extra_value(raw_links, &mut extra_values, idx);

    assert_eq!(result.value, 10);
    assert_eq!(extra_values.len(), 1);
}

