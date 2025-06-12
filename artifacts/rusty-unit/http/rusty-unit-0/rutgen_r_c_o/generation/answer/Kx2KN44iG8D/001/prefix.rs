// Answer 0

#[test]
fn test_from_iter_empty() {
    let iter: Vec<(HdrName, HeaderValue)> = Vec::new();
    let map = HeaderMap::from_iter(iter);
}

#[test]
fn test_from_iter_single_entry() {
    let iter: Vec<(HdrName, HeaderValue)> = vec![(HdrName::from_str("Header1").unwrap(), HeaderValue::from_str("Value1").unwrap())];
    let map = HeaderMap::from_iter(iter);
}

#[test]
fn test_from_iter_multiple_entries() {
    let iter: Vec<(HdrName, HeaderValue)> = vec![
        (HdrName::from_str("Header1").unwrap(), HeaderValue::from_str("Value1").unwrap()),
        (HdrName::from_str("Header2").unwrap(), HeaderValue::from_str("Value2").unwrap()),
        (HdrName::from_str("Header3").unwrap(), HeaderValue::from_str("Value3").unwrap()),
    ];
    let map = HeaderMap::from_iter(iter);
}

#[test]
fn test_from_iter_non_ascii_chars() {
    let iter: Vec<(HdrName, HeaderValue)> = vec![
        (HdrName::from_str("Héàder1").unwrap(), HeaderValue::from_str("Valué1").unwrap()),
        (HdrName::from_str("Héàder2").unwrap(), HeaderValue::from_str("Valué2").unwrap()),
    ];
    let map = HeaderMap::from_iter(iter);
}

#[test]
fn test_from_iter_large_number_of_entries() {
    let iter: Vec<(HdrName, HeaderValue)> = (0..32768)
        .map(|i| (HdrName::from_str(&format!("Header{}", i)).unwrap(), HeaderValue::from_str(&format!("Value{}", i)).unwrap()))
        .collect();
    let map = HeaderMap::from_iter(iter);
}

#[test]
fn test_from_iter_with_duplicate_keys() {
    let iter: Vec<(HdrName, HeaderValue)> = vec![
        (HdrName::from_str("Header1").unwrap(), HeaderValue::from_str("Value1").unwrap()),
        (HdrName::from_str("Header1").unwrap(), HeaderValue::from_str("Value2").unwrap()),
    ];
    let map = HeaderMap::from_iter(iter);
}

