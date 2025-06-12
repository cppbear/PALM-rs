// Answer 0

#[test]
fn test_value_iter_valid_index() {
    let mut map = HeaderMap::with_capacity(10);
    map.entries.push(Bucket {
        hash: 1,
        key: HeaderName::from("test1"),
        value: HeaderValue::from("value1"),
        links: None,
    });
    map.entries.push(Bucket {
        hash: 2,
        key: HeaderName::from("test2"),
        value: HeaderValue::from("value2"),
        links: None,
    });
    map.entries.push(Bucket {
        hash: 3,
        key: HeaderName::from("test3"),
        value: HeaderValue::from("value3"),
        links: None,
    });

    let idx = 1;
    let iter = map.value_iter(Some(idx));
}

#[test]
fn test_value_iter_edge_case_first_index() {
    let mut map = HeaderMap::with_capacity(5);
    map.entries.push(Bucket {
        hash: 1,
        key: HeaderName::from("edge1"),
        value: HeaderValue::from("value1"),
        links: None,
    });

    let idx = 0;
    let iter = map.value_iter(Some(idx));
}

#[test]
fn test_value_iter_edge_case_last_index() {
    let mut map = HeaderMap::with_capacity(5);
    map.entries.push(Bucket {
        hash: 1,
        key: HeaderName::from("edgeLast"),
        value: HeaderValue::from("lastValue"),
        links: None,
    });
    let idx = 0;
    let iter = map.value_iter(Some(idx));
}

#[test]
fn test_value_iter_non_empty_with_multiple_entries() {
    let mut map = HeaderMap::with_capacity(10);
    for i in 0..5 {
        map.entries.push(Bucket {
            hash: i as u64,
            key: HeaderName::from(format!("key{}", i)),
            value: HeaderValue::from(format!("value{}", i)),
            links: None,
        });
    }

    let idx = 4;
    let iter = map.value_iter(Some(idx));
}

