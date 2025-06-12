// Answer 0

#[test]
fn test_try_entry2_with_initial_capacity() {
    let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(16);
    let key = HeaderName { inner: Repr::<Custom>::default() };
    let hash = HashValue(1);
    map.indices = vec![Pos::none(); 16].into_boxed_slice();
    map.entries.push(Bucket {
        hash,
        key: key.clone(),
        value: HeaderValue::default(),
        links: None,
    });
    let probe = 0;
    let danger = false;

    let _ = map.try_entry2(key);
}

#[test]
fn test_try_entry2_with_load_factor_threshold() {
    let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(8);
    let key = HeaderName { inner: Repr::<Custom>::default() };
    let hash = HashValue(10);
    map.indices = vec![Pos::none(); 8].into_boxed_slice();
    map.entries.push(Bucket {
        hash,
        key: key.clone(),
        value: HeaderValue::default(),
        links: None,
    });
    let probe = 1;

    let _ = map.try_entry2(key);
}

#[test]
fn test_try_entry2_with_multiple_entries() {
    let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(32);
    let key = HeaderName { inner: Repr::<Custom>::default() };
    let hash = HashValue(2);
    map.indices = vec![Pos::none(); 32].into_boxed_slice();
    map.entries.push(Bucket {
        hash,
        key: key.clone(),
        value: HeaderValue::default(),
        links: None,
    });
    map.entries.push(Bucket {
        hash: HashValue(3),
        key: HeaderName { inner: Repr::<Custom>::default() },
        value: HeaderValue::default(),
        links: None,
    });
    let probe = 2;

    let _ = map.try_entry2(key);
}

#[test]
fn test_try_entry2_edge_case_max_size() {
    let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(32768);
    let key = HeaderName { inner: Repr::<Custom>::default() };
    let hash = HashValue(32767);
    map.indices = vec![Pos::none(); 32768].into_boxed_slice();
    map.entries.push(Bucket {
        hash,
        key: key.clone(),
        value: HeaderValue::default(),
        links: None,
    });
    let probe = 32767;

    let _ = map.try_entry2(key);
}

#[test]
fn test_try_entry2_on_occupied_spot() {
    let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(10);
    let key = HeaderName { inner: Repr::<Custom>::default() };
    let hash = HashValue(5);
    map.indices = vec![Pos::none(); 10].into_boxed_slice();
    map.entries.push(Bucket {
        hash,
        key: key.clone(),
        value: HeaderValue::default(),
        links: None,
    });
    map.entries.push(Bucket {
        hash: HashValue(6),
        key: key.clone(),
        value: HeaderValue::default(),
        links: None,
    });
    let probe = 3;

    let _ = map.try_entry2(key);
}

