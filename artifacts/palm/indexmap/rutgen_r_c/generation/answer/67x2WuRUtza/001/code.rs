// Answer 0

#[test]
fn test_split_first_with_non_empty_slice() {
    let first_bucket = Bucket { hash: HashValue::default(), key: 1, value: "first" };
    let second_bucket = Bucket { hash: HashValue::default(), key: 2, value: "second" };
    
    let slice = Box::new(Slice { entries: [first_bucket, second_bucket] });
    
    let result = slice.split_first();
    
    assert!(result.is_some());
    if let Some(((key, value), rest)) = result {
        assert_eq!(*key, 1);
        assert_eq!(*value, "first");
        assert_eq!(rest.len(), 1);
        assert_eq!(rest.get_index(0).map(|(k, v)| (*k, *v)), Some((2, "second")));
    }
}

#[test]
fn test_split_first_with_empty_slice() {
    let slice = Box::new(Slice { entries: [] });
    
    let result = slice.split_first();
    
    assert!(result.is_none());
}

#[test]
fn test_split_first_with_one_element_slice() {
    let only_bucket = Bucket { hash: HashValue::default(), key: 3, value: "only" };
    let slice = Box::new(Slice { entries: [only_bucket] });
    
    let result = slice.split_first();
    
    assert!(result.is_some());
    if let Some(((key, value), rest)) = result {
        assert_eq!(*key, 3);
        assert_eq!(*value, "only");
        assert_eq!(rest.len(), 0);
    }
}

