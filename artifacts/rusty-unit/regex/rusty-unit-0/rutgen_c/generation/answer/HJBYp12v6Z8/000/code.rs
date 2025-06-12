// Answer 0

#[test]
fn test_no_expansion_borrowed() {
    let data: &[u8] = b"test data";
    let mut no_expand = NoExpand(data);
    
    let result = no_expand.no_expansion();
    assert!(result.is_some());
    assert_eq!(result.unwrap(), Cow::Borrowed(data));
}

#[test]
fn test_no_expansion_empty() {
    let data: &[u8] = b"";
    let mut no_expand = NoExpand(data);
    
    let result = no_expand.no_expansion();
    assert!(result.is_some());
    assert_eq!(result.unwrap(), Cow::Borrowed(data));
}

