// Answer 0

#[test]
fn test_len_empty() {
    let map: HeaderMap = HeaderMap::with_capacity(10);
    map.len();
}

#[test]
fn test_len_single_entry() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("Header1".parse().unwrap(), "Value1".parse().unwrap());
    map.len();
}

#[test]
fn test_len_multiple_entries_same_key() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("Header1".parse().unwrap(), "Value1".parse().unwrap());
    map.append("Header1".parse().unwrap(), "Value2".parse().unwrap());
    map.len();
}

#[test]
fn test_len_multiple_unique_entries() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("Header1".parse().unwrap(), "Value1".parse().unwrap());
    map.insert("Header2".parse().unwrap(), "Value2".parse().unwrap());
    map.len();
}

#[test]
fn test_len_with_extra_values() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("Header1".parse().unwrap(), "Value1".parse().unwrap());
    map.append("Header1".parse().unwrap(), "Value2".parse().unwrap());
    map.append("Header1".parse().unwrap(), "Value3".parse().unwrap());
    map.len();
}

#[test]
fn test_len_full_capacity() {
    let mut map: HeaderMap = HeaderMap::with_capacity(32768);
    for i in 0..32768 {
        map.insert(format!("Header{}", i).parse().unwrap(), format!("Value{}", i).parse().unwrap());
    }
    map.len();
}

#[test]
fn test_len_exceeding_capacities() {
    let mut map: HeaderMap = HeaderMap::with_capacity(32768);
    map.insert("Header1".parse().unwrap(), "Value1".parse().unwrap());
    map.append("Header1".parse().unwrap(), "Value2".parse().unwrap());
    map.append("Header2".parse().unwrap(), "Value3".parse().unwrap());
    for i in 0..100 {
        map.append("Header3".parse().unwrap(), format!("Value{}", i).parse().unwrap());
    }
    map.len();
}

