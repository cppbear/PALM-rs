// Answer 0

#[test]
fn test_len_empty_map() {
    let map = http::HeaderMap::new();
    assert_eq!(0, map.len());
}

#[test]
fn test_len_single_value() {
    use http::header::{ACCEPT, HOST};

    let mut map = http::HeaderMap::new();
    map.insert(ACCEPT, "text/plain".parse().unwrap());
    assert_eq!(1, map.len());

    map.insert(HOST, "localhost".parse().unwrap());
    assert_eq!(2, map.len());
}

#[test]
fn test_len_multiple_values_for_key() {
    use http::header::ACCEPT;

    let mut map = http::HeaderMap::new();
    map.insert(ACCEPT, "text/plain".parse().unwrap());
    map.append(ACCEPT, "text/html".parse().unwrap());
    assert_eq!(2, map.len());
}

#[test]
fn test_len_edge_case_same_key() {
    use http::header::{ACCEPT, HOST};

    let mut map = http::HeaderMap::new();
    map.insert(ACCEPT, "text/plain".parse().unwrap());
    map.append(ACCEPT, "text/html".parse().unwrap());
    map.insert(HOST, "localhost".parse().unwrap());
    assert_eq!(3, map.len());
}

