// Answer 0

#[test]
fn test_iter_empty() {
    let map: HeaderMap = HeaderMap::with_capacity(10);
    let mut iter = map.iter();
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_single() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    let key = HeaderName::from_static("Key1");
    let value = HeaderValue::from_static("Value1");
    map.insert(key.clone(), value.clone());

    let mut iter = map.iter();
    assert_eq!(iter.next(), Some((key.clone(), value.clone())));
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_multiple() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    let key = HeaderName::from_static("Key2");
    let value1 = HeaderValue::from_static("Value1");
    let value2 = HeaderValue::from_static("Value2");

    map.insert(key.clone(), value1.clone());
    map.append(key.clone(), value2.clone());

    let mut iter = map.iter();
    assert_eq!(iter.next(), Some((key.clone(), value1.clone())));
    assert_eq!(iter.next(), Some((key.clone(), value2.clone())));
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_order() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    let key1 = HeaderName::from_static("Key3");
    let value1 = HeaderValue::from_static("Value1");
    let key2 = HeaderName::from_static("Key4");
    let value2 = HeaderValue::from_static("Value2");

    map.insert(key1.clone(), value1.clone());
    map.append(key1.clone(), value2.clone());
    map.insert(key2.clone(), HeaderValue::from_static("Value3"));

    let mut keys_values = vec![];
    let mut iter = map.iter();
    while let Some((key, value)) = iter.next() {
        keys_values.push((key.clone(), value.clone()));
    }

    assert_eq!(keys_values.len(), 3);
    assert!(keys_values.contains(&(key1.clone(), value1.clone())));
    assert!(keys_values.contains(&(key1.clone(), value2.clone())));
    assert!(keys_values.contains(&(key2.clone(), HeaderValue::from_static("Value3"))));
}

