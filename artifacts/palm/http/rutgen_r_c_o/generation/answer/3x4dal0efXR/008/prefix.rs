// Answer 0

#[test]
fn test_try_reserve_one_case_1() {
    let mut header_map = HeaderMap::with_capacity(16);
    header_map.entries.push(Bucket {
        hash: HashValue(0),
        key: HeaderName::from_static("Key1"),
        value: HeaderValue::from_static("Value1"),
        links: None,
    });
    header_map.danger.set_green();
    header_map.try_reserve_one().unwrap();
}

#[test]
fn test_try_reserve_one_case_2() {
    let mut header_map = HeaderMap::with_capacity(32);
    for i in 0..15 {
        header_map.entries.push(Bucket {
            hash: HashValue(i),
            key: HeaderName::from_static(&format!("Key{}", i)),
            value: HeaderValue::from_static(&format!("Value{}", i)),
            links: None,
        });
    }
    header_map.danger.set_green();
    header_map.try_reserve_one().unwrap();
}

#[test]
fn test_try_reserve_one_case_3() {
    let mut header_map = HeaderMap::with_capacity(64);
    for i in 0..31 {
        header_map.entries.push(Bucket {
            hash: HashValue(i),
            key: HeaderName::from_static(&format!("Key{}", i)),
            value: HeaderValue::from_static(&format!("Value{}", i)),
            links: None,
        });
    }
    header_map.danger.set_green();
    header_map.try_reserve_one().unwrap();
} 

#[test]
fn test_try_reserve_one_case_4() {
    let mut header_map = HeaderMap::with_capacity(128);
    for i in 0..63 {
        header_map.entries.push(Bucket {
            hash: HashValue(i),
            key: HeaderName::from_static(&format!("Key{}", i)),
            value: HeaderValue::from_static(&format!("Value{}", i)),
            links: None,
        });
    }
    header_map.danger.set_green();
    header_map.try_reserve_one().unwrap();
} 

#[test]
fn test_try_reserve_one_case_5() {
    let mut header_map = HeaderMap::with_capacity(256);
    for i in 0..127 {
        header_map.entries.push(Bucket {
            hash: HashValue(i),
            key: HeaderName::from_static(&format!("Key{}", i)),
            value: HeaderValue::from_static(&format!("Value{}", i)),
            links: None,
        });
    }
    header_map.danger.set_green();
    header_map.try_reserve_one().unwrap();
}

