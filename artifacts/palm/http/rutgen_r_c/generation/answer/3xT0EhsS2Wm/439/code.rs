// Answer 0

#[test]
fn test_remove_extra_value_with_entry_to_entry_link() {
    #[derive(Debug)]
    struct TestBucket;

    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(TestBucket), None, Some(TestBucket)])) as *mut [Option<TestBucket>]);
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(0) }, // Entry linking to itself
        ExtraValue { value: 2, prev: Link::Entry(1), next: Link::Entry(1) }, // Entry linking to itself
    ];
    
    let removed_value = remove_extra_value(raw_links, &mut extra_values, 0);
    
    assert_eq!(removed_value.value, 1); 
    assert_eq!(extra_values.len(), 1); 
}

#[test]
fn test_remove_extra_value_with_entry_to_extra_link() {
    #[derive(Debug)]
    struct TestBucket;

    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(TestBucket), Some(TestBucket), None])) as *mut [Option<TestBucket>]);
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Extra(1) }, // Links to 2nd Extra
        ExtraValue { value: 2, prev: Link::Extra(0), next: Link::Entry(1) }, // Links to Entry
    ];
    
    let removed_value = remove_extra_value(raw_links, &mut extra_values, 0);
    
    assert_eq!(removed_value.value, 1);
    assert_eq!(extra_values.len(), 1); 
}

#[test]
fn test_remove_extra_value_with_extra_to_entry_link() {
    #[derive(Debug)]
    struct TestBucket;

    let mut raw_links = RawLinks(Box::into_raw(Box::new([None, Some(TestBucket), Some(TestBucket)])) as *mut [Option<TestBucket>]);
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Extra(1), next: Link::Entry(0) }, // Links to Entry
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Extra(0) }, // Links to itself
    ];
    
    let removed_value = remove_extra_value(raw_links, &mut extra_values, 1);
    
    assert_eq!(removed_value.value, 2);
    assert_eq!(extra_values.len(), 1); 
}

#[test]
fn test_remove_extra_value_with_extra_to_extra_link() {
    #[derive(Debug)]
    struct TestBucket;

    let mut raw_links = RawLinks(Box::into_raw(Box::new([None, None, None])) as *mut [Option<TestBucket>]);
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Extra(1), next: Link::Extra(2) }, // Links among Extras
        ExtraValue { value: 2, prev: Link::Extra(0), next: Link::Extra(2) }, // Links to the first Extra
        ExtraValue { value: 3, prev: Link::Extra(0), next: Link::Extra(1) }, // Links to the first Extra
    ];
    
    let removed_value = remove_extra_value(raw_links, &mut extra_values, 0);
    
    assert_eq!(removed_value.value, 1);
    assert_eq!(extra_values.len(), 2);
}

#[test]
#[should_panic]
fn test_remove_extra_value_panic_due_to_idx() {
    #[derive(Debug)]
    struct TestBucket;

    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(TestBucket)])) as *mut [Option<TestBucket>]);
    let mut extra_values = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(0) },
    ];

    let _removed_value = remove_extra_value(raw_links, &mut extra_values, 1); // idx 1 does not exist
}

