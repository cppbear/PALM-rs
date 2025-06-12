// Answer 0

#[test]
fn test_iter_with_single_value() {
    struct TestHeaderValue;
    
    let mut map = HeaderMap::new();
    map.insert("host", TestHeaderValue);
    
    let values = map.get_all("host");
    let mut iter = values.iter();
    
    assert_eq!(iter.next().is_some(), true);
    assert_eq!(iter.next().is_none(), true);
}

#[test]
fn test_iter_with_multiple_values() {
    struct TestHeaderValue;

    let mut map = HeaderMap::new();
    map.insert("host", TestHeaderValue);
    map.append("host", TestHeaderValue);
    
    let values = map.get_all("host");
    let mut iter = values.iter();
    
    assert_eq!(iter.next().is_some(), true);
    assert_eq!(iter.next().is_some(), true);
    assert_eq!(iter.next().is_none(), true);
}

#[test]
fn test_iter_with_no_values() {
    struct TestHeaderValue;

    let mut map = HeaderMap::new();
    
    let values = map.get_all("host");
    let mut iter = values.iter();
    
    assert_eq!(iter.next().is_none(), true);
}

